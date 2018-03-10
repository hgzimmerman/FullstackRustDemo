use db::Conn;
use auth::hash_password;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use std::ops::Deref;
use chrono::{NaiveDateTime, Utc, Duration};
use schema::users;

use requests_and_responses::user::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[PgType = "Userrole"]
pub enum UserRole {
    Unprivileged,
    Moderator,
    Admin,
    Publisher,
}

impl From<UserRole> for i32 {
    fn from(role: UserRole) -> i32 {
        match role {
            UserRole::Unprivileged => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3,
            UserRole::Publisher => 4,
        }
    }
}

impl From<i32> for UserRole {
    fn from(number: i32) -> UserRole {
        match number {
            1 => UserRole::Unprivileged,
            2 => UserRole::Moderator,
            3 => UserRole::Admin,
            4 => UserRole::Publisher,
            _ => {
                warn!("Tried to convert an unsupported number into a user role");
                UserRole::Unprivileged
            }
        }
    }
}


impl From<User> for UserResponse {
    fn from(user: User) -> UserResponse {
        UserResponse {
            user_name: user.user_name,
            display_name: user.display_name,
            id: user.id,
        }
    }
}


impl From<NewUserRequest> for NewUser {
    fn from(new_user_request: NewUserRequest) -> NewUser {
        NewUser {
            user_name: new_user_request.user_name,
            display_name: new_user_request.display_name,
            password_hash: hash_password(&new_user_request.plaintext_password)
                .expect("Couldn't hash password"),
            failed_login_count: 0,
            banned: false,
            roles: vec![1],
        }
    }
}

impl From<User> for FullUserResponse {
    fn from(user: User) -> FullUserResponse {
        FullUserResponse {
            user_name: user.user_name,
            display_name: user.display_name,
            id: user.id,
            banned: user.banned,
            locked: user.locked.is_some(),
        }
    }
}


/// The database's representation of a user.
#[derive(Debug, Clone, Identifiable, Queryable, Crd, ErrorHandler)]
#[insertable = "NewUser"]
#[table_name = "users"]
pub struct User {
    /// The primary key
    pub id: i32,
    /// The user name of the user. This is used primairily for logging in, and is seldom displayed.
    pub user_name: String,
    /// This name will be displayed on data associated with the user, such as forum posts, or as the author of articles.
    pub display_name: String,
    /// The stored hash of the password.
    pub password_hash: String,
    /// If the user is locked, they cannot try to log in until the timer expires.
    /// If the user fails a password attempt, lock them out for n seconds.
    pub locked: Option<NaiveDateTime>,
    pub failed_login_count: i32,
    /// If the user is banned, they cannot log in without being unbanned.
    pub banned: bool,
    /// The roles of the user.
    pub roles: Vec<i32>, // currently this is stored as an int. It would be better to store it as an enum, if diesel-enum serialization can be made to work.
}


#[derive(Insertable, Debug, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
    pub display_name: String,
    pub password_hash: String,
    pub failed_login_count: i32,
    pub banned: bool,
    pub roles: Vec<i32>,
    // pub locked: Option<NaiveDateTime>,
}


impl User {
    /// Gets the user by their user name.
    pub fn get_user_by_user_name(name: &str, conn: &Conn) -> JoeResult<User> {
        use schema::users::dsl::*;
        info!("Getting user with Name: {}", name);

        users
            .filter(user_name.eq(user_name))
            .first::<User>(conn.deref())
            .map_err(User::handle_error)
    }



    /// Gets a vector of users of length n.
    // TODO: consider also specifing a step, so that this can be used in a proper pagenation system.
    pub fn get_users(num_users: i64, conn: &Conn) -> JoeResult<Vec<User>> {
        use schema::users::dsl::*;
        users
            .limit(num_users)
            .load::<User>(conn.deref())
            .map_err(User::handle_error)
    }

    /// For the given role, get all users with the that role.
    pub fn get_users_with_role(user_role: UserRole, conn: &Conn) -> JoeResult<Vec<User>> {

        let user_role_id: i32 = i32::from(user_role);

        User::get_all(conn).map(|users| {
            users
                .into_iter()
                .filter(|user| user.roles.contains(&user_role_id))
                .collect()
        })
    }

    /// If the user has their banned flag set, this will return true.
    pub fn is_user_banned(user_id: i32, conn: &Conn) -> JoeResult<bool> {
        use schema::users::dsl::*;
        users
            .find(user_id)
            .select(banned)
            .first::<bool>(conn.deref())
            .map_err(User::handle_error)
    }

    // TODO, refactor this, only implement the db transaction, logic can go in the login method
    pub fn check_if_locked(&self, conn: &Conn) -> JoeResult<bool> {
        use schema::users::dsl::*;

        if let Some(l) = self.locked {
            let current_date = Utc::now().naive_utc();
            if current_date > l {
                Ok(true)
            } else {
                // Remove the locked status
                let target = users.filter(id.eq(self.id));
                diesel::update(target)
                    .set(locked.eq(None::<NaiveDateTime>))
                    .execute(conn.deref())
                    .map_err(User::handle_error)?;
                Ok(false)
            }
        } else {
            // No need to remove a lock status that isn't present.
            Ok(false)
        }
    }

    /// Resets the login failure count to 0.
    /// This should be called after the user logs in successfully.
    pub fn reset_login_failure_count(user_id: i32, conn: &Conn) -> JoeResult<()> {
        use schema::users::dsl::*;
        let target = users.filter(id.eq(user_id));
        diesel::update(target)
            .set(failed_login_count.eq(0))
            .execute(conn.deref())
            .map_err(User::handle_error)?;
        Ok(())
    }

    /// This method is to be called after a user has failed to log in.
    /// Based on the number of current failed login attempts in a row, it will calculate the locked period.
    /// It will then store the datetime of unlock, along with an incremented failure count, so that next time it will take longer.
    pub fn record_failed_login(user_id: i32, current_failed_attempts: i32, conn: &Conn) -> JoeResult<NaiveDateTime> {
        use schema::users::dsl::*;

        info!("record_failed_login: setting the expire time and failure count");
        let current_date = Utc::now().naive_utc();
        let delay_seconds: i64 = (current_failed_attempts * 2).into(); // Todo: come up with a better function than this
        let expire_datetime = current_date + Duration::seconds(delay_seconds);

        let target = users.filter(id.eq(user_id));
        let _ = diesel::update(target)
            .set((
                locked.eq(expire_datetime),
                failed_login_count.eq(
                    current_failed_attempts +
                        1,
                ), // Increment the failed count
            ))
            .execute(conn.deref())
            .map_err(User::handle_error)?;

        return Ok(expire_datetime);
    }

    /// Adds a role to the user.
    pub fn add_role_to_user(user_id: i32, user_role: UserRole, conn: &Conn) -> JoeResult<User> {

        use schema::users::dsl::*;

        let user = User::get_by_id(user_id, conn)?;

        let user_role_id: i32 = i32::from(user_role);
        if user.roles.contains(&user_role_id) {
            // The user already has the id, no need to assign it again.
            return Ok(user);
        } else {
            // Because the user does not have the role, it needs to be added to to its list
            let mut new_roles = user.roles.clone();
            new_roles.push(user_role_id);

            let target = users.filter(id.eq(user_id));
            diesel::update(target)
                .set(roles.eq(new_roles))
                .get_result(conn.deref())
                .map_err(User::handle_error)
        }
    }


    /// Gets a number of users at specified offsets.
    pub fn get_paginated(page_index: i32, page_size: i32, conn: &Conn) -> JoeResult<(Vec<User>, i64)> {
        use schema::users::dsl::*;
        use schema::users;
        use db::diesel_extensions::pagination::Paginate;

        users::table
            .filter(id.gt(0)) // NoOp filter to get the paginate function to work.
            .paginate(page_index.into())
            .per_page(page_size.into())
            .load_and_count_pages::<User>(conn.deref())
            .map_err(User::handle_error)
    }

    /// Updates the user's display name.
    pub fn update_user_display_name(request: UpdateDisplayNameRequest, conn: &Conn) -> JoeResult<User> {

        use schema::users::dsl::*;
        let target = users.filter(
            user_name.eq(request.user_name),
        );

        info!("Updating the user display name");
        diesel::update(target)
            .set(display_name.eq(
                request.new_display_name,
            ))
            .get_result(conn.deref())
            .map_err(User::handle_error)
    }

    /// Deletes the user by their name.
    pub fn delete_user_by_name(name: String, conn: &Conn) -> JoeResult<User> {
        use schema::users::dsl::*;

        let target = users.filter(user_name.eq(name));

        diesel::delete(target)
            .get_result(conn.deref())
            .map_err(User::handle_error)
    }
}


#[cfg(test)]
mod test {
    use db;
    use db::Conn;
    use super::*;


    #[test]
    fn crud() {

        let pool = db::init_pool();

        let user_name: String = "CrudTest-UserName".into();

        // Delete the entry to avoid
        let conn = Conn::new(pool.get().unwrap());
        let _ = User::delete_user_by_name(user_name.clone(), &conn);

        // Create a user
        let new_user = NewUserRequest {
            user_name: user_name.clone(),
            display_name: "DisplayName".into(),
            plaintext_password: "TestPassword".into(),
        };
        let response: UserResponse = User::create(new_user.into(), &conn)
            .unwrap()
            .into();
        assert_eq!(user_name.clone(), response.user_name);

        // Get User
        let response: UserResponse = User::get_by_id(response.id, &conn)
            .unwrap()
            .into();
        assert_eq!(user_name.clone(), response.user_name);


        // Modify user
        let update_display_name_request: UpdateDisplayNameRequest = UpdateDisplayNameRequest {
            user_name: user_name.clone(),
            new_display_name: "NewDisplayName".into(),
        };
        let response: UserResponse = User::update_user_display_name(update_display_name_request, &conn)
            .unwrap()
            .into();
        assert_eq!("NewDisplayName".to_string(), response.display_name);


        // Delete the entry
        let _ = User::delete_user_by_name(user_name, &conn);
    }

    #[test]
    fn orphan_test() {
        use db::article::{NewArticle, Article};

        let pool = db::init_pool();

        let user_name: String = String::from("OrphanTest-UserName");

        let conn = Conn::new(pool.get().unwrap());
        // Article::delete_by_id(2, &conn).unwrap();
        // println!("{:?}", Article::get_all(&conn));
        let _ = User::delete_user_by_name(user_name.clone(), &conn);

        // panic!("Expected fail");
        let new_user: NewUser = NewUserRequest {
            user_name: user_name.clone(),
            display_name: String::from("DisplayName"),
            plaintext_password: String::from("TestPassword")
        }.into();

        let user = User::create(new_user, &conn).unwrap();

        let new_article: NewArticle = NewArticle {
            title: String::from("OrphanTest-ArticleTitle"),
            slug: String::from("aah"),
            body: String::from("body"),
            author_id: user.id
        };

        let child_article: Article = Article::create(new_article, &conn).unwrap();

        // The user should not be able to be deleted because an article references it.
        assert!(User::delete_by_id(user.id, &conn).is_err(), true);

        Article::delete_by_id(child_article.id, &conn).expect("Expected to be able to delete article.");
        User::delete_by_id(user.id, &conn).expect("Should be able to delete user after dependent article is deleted.");
    }

}

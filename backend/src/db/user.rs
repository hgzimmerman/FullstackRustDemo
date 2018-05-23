use db::Conn;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use std::ops::Deref;
use chrono::{NaiveDateTime, Utc, Duration};
use schema::users;
use error::JoeResult;
use log;

// TODO, I don't think that this file should have wire types
use wire::user::*;


/// The database's representation of a user.
#[derive(Debug, Clone, Identifiable, Queryable, Crd, ErrorHandler)]
#[insertable = "NewUser"]
#[table_name = "users"]
pub struct User {
    /// The primary key
    pub id: i32,
    /// The user name of the user. This is used primarily for logging in, and is seldom displayed.
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
        log::info!("Getting user with Name: {}", name);

        users
            .filter(user_name.eq(name))
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

        use schema::users::dsl::*;
        use schema::users;
        use diesel::PgArrayExpressionMethods;

        // Diesel can construct queries that operate on the contents of Postgres arrays.
        users
            .filter(users::roles.contains(vec![user_role_id]))
            .load::<User>(conn.deref())
            .map_err(User::handle_error)

        // This is inefficient because it loads the whole users table into memory to filter on the roles vector
//        User::get_all(conn).map(|users| {
//            users
//                .into_iter()
//                .filter(|user| user.roles.contains(&user_role_id))
//                .collect()
//        })
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
        use schema::users;
        let target = users.filter(users::id.eq(user_id));
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

        log::info!("record_failed_login: setting the expire time and failure count");
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

    pub fn set_ban_status(user_id: i32, is_banned: bool, conn: &Conn) -> JoeResult<User> {
        use schema::users::dsl::*;
        let target = users.filter(id.eq(user_id));
        diesel::update(target)
            .set(banned.eq(is_banned))
            .get_result(conn.deref())
            .map_err(User::handle_error)

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

        log::info!("Updating the user display name");
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
    use test::Bencher;


    #[bench]
    fn crud_bench(b: &mut Bencher) {
        let pool = db::init_pool();

        let conn = Conn::new(pool.get().unwrap());

        fn crud(conn: &Conn) {
            let user_name: String = "CrudBenchTest-UserName".into();

            // Delete the entry to avoid
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


        b.iter(|| crud(&conn))
    }

    #[bench]
    fn get_user_bench(b: &mut Bencher) {
        let pool = db::init_pool();

        let conn = Conn::new(pool.get().unwrap());

        let user_name: String = "GetBenchTest-UserName".into();

        // Delete the entry to avoid
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

        // perform the bench
        b.iter(
            || User::get_by_id(response.id, &conn),
        );

        // Delete the user
        let _ = User::delete_user_by_name(user_name, &conn);
    }


}

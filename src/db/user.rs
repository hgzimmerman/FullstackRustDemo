use db::Conn;
use auth::hash_password;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use std::ops::Deref;
// use chrono::NaiveDateTime;
use schema::users;

use requests_and_responses::user::*;
use error::WeekendAtJoesError;
use db::handle_diesel_error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[PgType = "Userrole"]  
pub enum UserRole {
    Unprivileged,
    Moderator,
    Admin,
    Publisher
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
            id: user.id
        }
    }
}


impl From<NewUserRequest> for NewUser {
    fn from(new_user_request: NewUserRequest) -> NewUser {
        NewUser {
            user_name: new_user_request.user_name,
            display_name: new_user_request.display_name,
            password_hash: hash_password(&new_user_request.plaintext_password).expect("Couldn't hash password"),
            // token_key: None,
            // token_expire_date: None,
            roles: vec![1]
        }
    }
}


/// The database's representation of a user.
#[derive(Debug, Clone, Identifiable, Queryable)]
#[table_name="users"]
pub struct User {
    /// The primary key
    pub id: i32,
    /// The user name of the user. This is used primairily for logging in, and is seldom displayed.
    pub user_name: String,
    /// This name will be displayed on data associated with the user, such as forum posts, or as the author of articles.
    pub display_name: String,
    /// The stored hash of the password.
    pub password_hash: String,
    /// The roles of the user.
    pub roles: Vec<i32> // currently this is stored as an int. It would be better to store it as an enum, if diesel-enum serialization can be made to work.
}


#[derive(Insertable, Debug, Clone)]
#[table_name="users"]
pub struct NewUser {
    pub user_name: String,
    pub display_name: String,
    pub password_hash: String,
    pub roles: Vec<i32>
}


impl User {

    /// Gets the user by their user name.
    pub fn get_user_by_user_name(name: &str, conn: &Conn) -> Result<User, WeekendAtJoesError> {
        use schema::users::dsl::*;
        info!("Getting user with Name: {}", name);

        users
            .filter(user_name.eq(user_name))
            .first::<User>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "User"))
    }

    /// Gets the user by their id.
    pub fn get_user(user_id: i32, conn: &Conn) -> Result<User, WeekendAtJoesError> {
        use schema::users::dsl::*;
        info!("Getting user with ID: {}", user_id);

        users.find(user_id)
            .first::<User>(conn.deref()) 
            .map_err(|e| handle_diesel_error(e, "User"))
    }

    /// Gets a vector of users of length n.
    // TODO: consider also specifing a step, so that this can be used in a proper pagenation system.
    pub fn get_users(num_users: i64, conn: &Conn) -> Result<Vec<User>, WeekendAtJoesError> {
        use schema::users::dsl::*;
        users
            .limit(num_users)
            .load::<User>(conn.deref())
            .map_err(|e| handle_diesel_error(e, "User"))
    }

    /// Creates a new user.
    pub fn create_user(new_user: NewUserRequest, conn: &Conn) -> Result<User, WeekendAtJoesError> {
        use schema::users;

        info!("Creating new user with the following values: {:?}", new_user);
        let new_user: NewUser = new_user.into();

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "User"))
    }
    
    /// Updates the user's display name.
    pub fn update_user_display_name(request: UpdateDisplayNameRequest, conn: &Conn) -> Result<User, WeekendAtJoesError> {

        use schema::users::dsl::*;
        let target = users.filter(user_name.eq(request.user_name));

        info!("Updating the user display name");
        diesel::update(target)
            .set(display_name.eq(request.new_display_name))
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "User"))
    }

    /// Deletes the user by id.
    pub fn delete_user_by_id(user_id: i32, conn: &Conn) -> Result<User, WeekendAtJoesError> {
        use schema::users::dsl::*;

        let target = users.filter(id.eq(user_id));

        diesel::delete(target)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "User"))
    }

    /// Deletes the user by their name.
    pub fn delete_user_by_name(name: String, conn: &Conn) -> Result<User, WeekendAtJoesError> {
        use schema::users::dsl::*;

        let target = users.filter(user_name.eq(name));

        diesel::delete(target)
            .get_result(conn.deref())
            .map_err(|e| handle_diesel_error(e, "User"))
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
            plaintext_password: "TestPassword".into() 
        };
        let response: UserResponse =  User::create_user(new_user, &conn).unwrap().into();
        assert_eq!(user_name.clone(), response.user_name);

        // Get User
        let response: UserResponse =  User::get_user(response.id, &conn).unwrap().into();
        assert_eq!(user_name.clone(), response.user_name);


        // Modify user
        let update_display_name_request: UpdateDisplayNameRequest = UpdateDisplayNameRequest {
            user_name: user_name.clone(),
            new_display_name: "NewDisplayName".into()
        };
        let response: UserResponse = User::update_user_display_name(update_display_name_request, &conn).unwrap().into();
        assert_eq!("NewDisplayName".to_string(), response.display_name);


        // Delete the entry
        let _ = User::delete_user_by_name(user_name, &conn);
    }

}
use db::Conn;
use auth::hash_password;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::result::Error;
use std::ops::Deref;
use chrono::NaiveDateTime;
use schema::users;

use requests_and_responses::user::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// #[PgType = "Userrole"]  
pub enum UserRole {
    Unprivileged,
    Moderator,
    Admin
}

impl From<UserRole> for i32 {
    fn from(role: UserRole) -> i32 {
        match role {
            UserRole::Unprivileged => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3
        }
    }
}

impl From<i32> for UserRole {
    fn from(number: i32) -> UserRole {
        match number {
            1 => UserRole::Unprivileged,
            2 => UserRole::Moderator,
            3 => UserRole::Admin,
            _ => {
                warn!("Tried to convert an unsupported number into a user role");
                UserRole::Unprivileged
            }
        }
    }
}


/// User to be stored in db.
/// This user will be used to check for auth.
#[derive( Debug, Clone, Identifiable, Queryable)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub display_name: String,
    pub password_hash: String,

    pub token_key: Option<String>,
    pub token_expire_date: Option<NaiveDateTime>,
    pub roles: Vec<i32>
}


#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="users"]
pub struct NewUser {
    pub user_name: String,
    pub display_name: String,
    pub password_hash: String,
    pub token_key: Option<String>,
    pub token_expire_date: Option<NaiveDateTime>,
    pub roles: Vec<i32>
}


impl User {
    pub fn get_user_by_user_name(name: &str, conn: &Conn) -> Option<User> {
        use schema::users::dsl::*;
        info!("Getting user with Name: {}", name);

        let returned_users: Vec<User> = users
            .filter(user_name.eq(user_name))
            .limit(1)
            .load::<User>(conn.deref())
            .expect("db error");

        return returned_users.get(0).map(|x| x.clone());
    }

    pub fn update_user_jwt(user_name: String, token_key: String, token_expire_date: NaiveDateTime, conn: &Conn ) -> Result<usize, Error> {
        use schema::users::dsl::*;
        use schema::users;
        // info!("Updating the display name of user id {} to {}", data.id, data.new_display_name);

        let target = users.filter(user_name.eq(user_name));

        let update_response = diesel::update(target)
            .set((
                users::token_key.eq(&token_key),
                users::token_expire_date.eq(&token_expire_date))
            )
            .execute(conn.deref());  
        update_response
    }

    pub fn get_user(user_id: i32, conn: &Conn) -> Option<User> {
        use schema::users::dsl::*;
        info!("Getting user with ID: {}", user_id);

        let returned_users: Vec<User> = match users
            .filter(id.eq(user_id))
            .limit(1)
            .load::<User>(conn.deref()) 
        {
             Ok(x) => x,
             Err(e) => {
                info!("get_user failed: {:?}", e);
                return None;
             }

        };

        returned_users.get(0).cloned()
    }

    pub fn create_user(new_user: NewUserRequest, conn: &Conn) -> Result<User, Error> {
        use schema::users;

        info!("Creating new user with the following values: {:?}", new_user);
        let new_user: NewUser = new_user.into();

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn.deref())
    }
    
    pub fn update_user_display_name(request: UpdateDisplayNameRequest, conn: &Conn) -> Result<User, Error> {

        use schema::users::dsl::*;
        let target = users.filter(id.eq(request.id));

        let updated_user: Result<User, Error> = diesel::update(target)
            .set(display_name.eq(request.new_display_name))
            .get_result(conn.deref());

        updated_user
    }

    pub fn delete_user_by_id(user_id: i32, conn: &Conn) -> Result<User, Error> {
        use schema::users::dsl::*;

        let target = users.filter(id.eq(user_id));

        diesel::delete(target)
            .get_result(conn.deref())
    }

    pub fn delete_user_by_name(user_name: String, conn: &Conn) -> Result<User, Error> {
        use schema::users::dsl::*;

        let target = users.filter(user_name.eq(user_name));

        diesel::delete(target)
            .get_result(conn.deref())
    }
}

#[cfg(test)]
mod test {
    use super::super::super::init_rocket; // initialize the webserver
    use rocket::local::Client;
    use rocket::http::Status;
    use rocket::http::ContentType;
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
            id: response.id,
            new_display_name: "NewDisplayName".into()
        };
        let response: UserResponse = User::update_user_display_name(update_display_name_request, &conn).unwrap().into();
        assert_eq!("NewDisplayName".to_string(), response.display_name);


        // Delete the entry
        let _ = User::delete_user_by_name(user_name, &conn);
    }

}
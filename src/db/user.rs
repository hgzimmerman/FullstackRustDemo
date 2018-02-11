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

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserRequest {
    pub user_name: String,
    pub display_name: String,
    pub plaintext_password: String
}

impl From<NewUserRequest> for NewUser {
    fn from(new_user_request: NewUserRequest) -> NewUser {
        NewUser {
            user_name: new_user_request.user_name,
            display_name: new_user_request.display_name,
            password_hash: hash_password(&new_user_request.plaintext_password).expect("Couldn't hash password"),
            token_key: None,
            token_expire_date: None,
            roles: vec![1]
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

    pub fn create_user(new_user: NewUserRequest, conn: &Conn) -> Result<User, diesel::result::Error> {
        use schema::users;

        info!("Creating new user with the following values: {:?}", new_user);
        let new_user: NewUser = new_user.into();

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn.deref())
        
    }
}
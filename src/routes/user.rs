use rocket::Route;
use rocket_contrib::Json;
// use auth::userpass::FromString;
// use bcrypt::{DEFAULT_COST, hash, verify, BcryptError};
use super::Routable;
use diesel;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use diesel::result::Error;
use chrono::{NaiveDateTime, Utc};
use db::Conn;

use auth::hash_password;
use std::ops::Deref;
use schema::users;

// #[derive(Serialize, Deserialize, Debug, DbEnum, Clone)]
// #[PgType = "Userrole"]  
// pub enum Userrole {
//     Unprivileged,
//     Moderator,
//     Admin
// }

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
    user_name: String,
    display_name: String,
    password_hash: String,
    roles: Vec<i32>
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
            roles: vec![1]
        }
    }
}



impl User {
    pub fn get_user_by_user_name(name: &str, conn: &Conn) -> Option<User> {
        use schema::users;
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
}


/// User to be sent over the wire
#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    user_name: String,
    display_name: String,
    id: i32,
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

#[get("/<user_id>")]
fn get_user(user_id: i32, conn: Conn) -> Option<Json<UserResponse>> {
    use schema::users;
    use schema::users::dsl::*;
    info!("Getting user with ID: {}", user_id);

    let returned_users: Vec<User> = users
        .filter(id.eq(user_id))
        .limit(1)
        .load::<User>(&*conn)
        .expect("db error");

    match returned_users.get(0) {
        Some(user) => {
            let user_response: UserResponse = user.clone().into();
            Some(Json(user_response))
        },
        None => None
    }
}




#[post("/", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUserRequest>, conn: Conn) -> Json<UserResponse> {
    use schema::users;

    info!("Creating new user with the following values: {:?}", new_user);
    let new_user: NewUser = new_user.into_inner().into();

    let inserted_user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&*conn)
        .expect("Couldn't create user");
    
    let user_response: UserResponse = inserted_user.into();
    Json(user_response)
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDisplayNameRequest {
    id: i32,
    new_display_name: String
}
#[put("/", data = "<data>")]
fn update_user_display_name(data: Json<UpdateDisplayNameRequest>, conn: Conn ) -> Option<Json<UserResponse>> {
    use schema::users::dsl::*;
    use schema::users;
    let data: UpdateDisplayNameRequest = data.into_inner();
    info!("Updating the display name of user id {} to {}", data.id, data.new_display_name);

    let target = users.filter(id.eq(data.id));

    let updated_user: Result<User, Error> = diesel::update(target)
        .set(display_name.eq(data.new_display_name))
        .get_result(&*conn);

    match updated_user {
        Ok(updated_user) => {
            let user_response: UserResponse = updated_user.into();
            Some(Json(user_response))
        }
        Err(_) => None
    }
    
}

// #[delete("/<user_id>")]
// fn tombstone_user(user_id: i32, conn: Conn) -> Option<Json<UserResponse>> {
//     use schema::users::dsl::*;
//     use schema::users;
//     info!("Tombstone the user id: {}.", user_id);

//     let target = users.filter(id.eq(user_id));

//     let updated_user: Result<User, Error> = diesel::update(target)
//         .set(tombstone.eq(true))
//         .get_result(&*conn);

//     match updated_user {
//         Ok(updated_user) => {
//             let user_response: UserResponse = updated_user.into();
//             Some(Json(user_response))
//         }
//         Err(_) => None
//     }
// }

/// Currently, this is not exposed as an API, but is useful in testing
#[delete("/<user_id>")]
fn delete_user(user_id: i32, conn: Conn) -> Option<Json<UserResponse>> {
    use schema::users::dsl::*;
    use schema::users;

    let target = users.filter(id.eq(user_id));

    let updated_user: Result<User, Error> = diesel::delete(target)
        .get_result(&*conn);

    match updated_user {
        Ok(updated_user) => {
            let user_response: UserResponse = updated_user.into();
            Some(Json(user_response))
        }
        Err(_) => None
    }
}

/// Currently, this is not exposed as an API, but is useful in testing
pub fn delete_user_by_name(user_name: String, conn: Conn) -> Option<Json<UserResponse>> {
    use schema::users::dsl::*;
    use schema::users;

    let target = users.filter(user_name.eq(user_name));

    let updated_user: Result<User, Error> = diesel::delete(target)
        .get_result(&*conn);

    match updated_user {
        Ok(updated_user) => {
            let user_response: UserResponse = updated_user.into();
            Some(Json(user_response))
        }
        Err(e) => {
            info!("Couldn't delete user. Reason: {}", e);
            None
        }
    }
}
// Export the ROUTES and their path
impl Routable for User {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![create_user, update_user_display_name, get_user, delete_user];
    const PATH: &'static str = "/user/";
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

        // Delete the entry to avoid 
        let conn = Conn::new(pool.get().unwrap());
        let response = delete_user_by_name("UserName".into(), conn);

        // Create a user
        let conn = Conn::new(pool.get().unwrap());
        let new_user = NewUserRequest {
            user_name: "UserName".into(),
            display_name: "DisplayName".into(),
            plaintext_password: "TestPassword".into() 
        };
        let response: UserResponse =  create_user(Json(new_user), conn).into_inner();
        assert_eq!("UserName".to_string(), response.user_name);

        // Get User
        let conn = Conn::new(pool.get().unwrap());
        let response: UserResponse =  get_user(response.id, conn).unwrap().into_inner();
        assert_eq!("UserName".to_string(), response.user_name);


        // Modify user
        let conn = Conn::new(pool.get().unwrap());
        let update_display_name_request: UpdateDisplayNameRequest = UpdateDisplayNameRequest {
            id: response.id,
            new_display_name: "NewDisplayName".into()
        };
        let response: UserResponse = update_user_display_name(Json(update_display_name_request), conn).unwrap().into_inner();
        assert_eq!("NewDisplayName".to_string(), response.display_name);


        // Delete the entry
        let conn = Conn::new(pool.get().unwrap());
        delete_user_by_name("UserName".into(), conn);
    }


}
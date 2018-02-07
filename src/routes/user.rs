use rocket::Route;
use rocket_contrib::Json;
use auth::userpass::FromString;
use bcrypt::{DEFAULT_COST, hash, BcryptError};
use super::Routable;
use diesel;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;

use db::Conn;

use schema::users;

// #[derive(Serialize, Deserialize, Debug)]
// pub enum UserRole {
//     Unprivileged,
//     Admin
// }

/// User to be stored in db.
/// This user will be used to check for auth.
#[derive(Serialize, Deserialize, Debug, Identifiable, Queryable)]
#[table_name="users"]
pub struct User {
    id: i32,
    user_name: String,
    display_name: String,
    password_hash: String,
    tombstone: bool
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="users"]
pub struct NewUser {
    user_name: String,
    display_name: String,
    password_hash: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserRequest {
    user_name: String,
    display_name: String,
    plaintext_password: String
}

impl From<NewUserRequest> for NewUser {
    fn from(new_user_request: NewUserRequest) -> NewUser {
        NewUser {
            user_name: new_user_request.user_name,
            display_name: new_user_request.display_name,
            password_hash: hash_password(new_user_request.plaintext_password).expect("Couldn't hash password")
        }
    }
}




fn hash_password(password: String) -> Result<String, BcryptError> {
    hash(password.as_str(), DEFAULT_COST)
}


// impl From<LoginUser> for User {
//     fn from(new_user: LoginUser) -> StoredUser {
//         let hashed_pw = hash_pw(new_user.password)
//         User {
//             name: new_user.name,
//             pw_hash: hashed_pw,
//             id: Uuid::new_v4().hyphenated().to_string(),
//             user_roles: vec![UserRole::Unprivileged]
//         }
//     }
// }

/// Used for logging in and creating accounts.
#[derive(Serialize, Deserialize, Debug)]
struct LoginUser {
    user_name: String,
    plaintext_password: String 
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
fn get_user(user_id: String) -> Json<UserResponse> {
    info!("Getting user with ID: {}", user_id);
    //TODO find the user in the DB using the id
    unimplemented!();
}

#[post("/", data = "<new_user>")]
fn create_user(new_user: Json<NewUserRequest>, conn: Conn) -> Json<UserResponse> {
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

#[put("/", data = "<user>")]
fn update_user(user: Json<User>) -> Json<UserResponse> {
    info!("Updating user with the following values: {:?}", user);
    // let user: User = user.into_inner();
    // Json(user)
    unimplemented!()
}

#[delete("/<user_id>")]
fn delete_user(user_id: String) -> Json<UserResponse> {
    info!("Deleting user with ID: {}", user_id);
    // Json(UserResponse {
    //     display_name: String::from("test"),
    //     id: user_id,
    // })
    unimplemented!();
    // tombstone instead of delete please
}

// Export the ROUTES and their path
impl Routable for User {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![create_user, update_user, get_user, delete_user];
    const PATH: &'static str = "/user/";
}


#[cfg(test)]
mod test {
    use super::super::super::init_rocket; // initialize the webserver
    use rocket::local::Client;
    use rocket::http::Status;
    use super::*;

    #[test]
    fn get_user() {
        let client = Client::new(init_rocket()).expect("valid rocket instance");
        let mut response = client.get("/api/user/some_uuid_or_something").dispatch();
        assert_eq!(response.status(), Status::Ok);
//        assert_eq!(
//            Json(json!(response.body_string().unwrap())),
//            Json( User {
//                name: String::from("test"),
//                id: "some_uuid_or_something".to_string(),
//            }).
//        );
    }
}
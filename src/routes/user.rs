use rocket::Route;
use rocket_contrib::Json;
use uuid::Uuid;
use auth::userpass::FromString;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use bcrypt::{DEFAULT_COST, hash};

#[derive(Serialize, Deserialize, Debug)]
pub enum UserRole {
    Unprivileged,
    Admin
}

/// User to be stored in db.
/// This user will be used to check for auth.
#[derive(Serialize, Deserialize, Debug)]
pub struct StoredUser {
    name: String,
    pw_hash: String,
    id: String, // Uuid
    user_roles: Vec<UserRole>
}


impl StoredUser {
    fn new(name: String, password: String ) -> StoredUser {
        let hashed_pw = hash(password.as_str(), DEFAULT_COST).expect(format!("Hashing Password failed for user name: {}", name).as_str() );
        StoredUser {
            name: name,
            pw_hash: hashed_pw,
            id: Uuid::new_v4().hyphenated().to_string(),
            user_roles: vec![UserRole::Unprivileged]
        }
    }
}

impl FromString for StoredUser {
    fn from_string(s: String) -> Self {
        return StoredUser {
            name: String::from(""),
            pw_hash: String::from(""),
            id: s,
            user_roles: vec![]
        }
    }
}

impl From<LoginUser> for StoredUser {
    fn from(new_user: LoginUser) -> StoredUser {
        let hashed_pw = hash(new_user.password.as_str(), DEFAULT_COST).expect(format!("Hashing Password failed for user name: {}", new_user.name).as_str() );
        StoredUser {
            name: new_user.name,
            pw_hash: hashed_pw,
            id: Uuid::new_v4().hyphenated().to_string(),
            user_roles: vec![UserRole::Unprivileged]
        }
    }
}

/// Used for logging in and creating accounts.
#[derive(Serialize, Deserialize, Debug)]
struct LoginUser {
    name: String,
    password: String
}


/// User to be sent over the wire
#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    id: String,
}
impl From<StoredUser> for User {
    fn from(stored_user: StoredUser) -> User {
        User {
            name: stored_user.name,
            id: stored_user.id
        }
    }
}

#[get("/<user_id>")]
fn get_user(user_id: String) -> Json<User> {
    info!("Getting user with ID: {}", user_id);
    Json(User {
        name: String::from("test"),
        id: user_id,
    })
}

#[post("/", data = "<new_user>")]
fn create_user(new_user: Json<LoginUser>) -> Json<StoredUser> { // TODO don't actually return the stored user
    info!("Creating new user with the following values: {:?}", new_user);
    let new_user: LoginUser = new_user.into_inner();
    Json(StoredUser::from(new_user))
}

#[put("/", data = "<user>")]
fn update_user(user: Json<User>) -> Json<User> {
    info!("Updating user with the following values: {:?}", user);
    let user: User = user.into_inner();
    Json(user)
}

#[delete("/<user_id>")]
fn delete_user(user_id: String) -> Json<User> {
    info!("Deleting user with ID: {}", user_id);
    Json(User {
        name: String::from("test"),
        id: user_id,
    })
}

// Export the routes and their path
pub fn user_routes() -> Vec<Route> {
    routes![create_user, update_user, get_user, delete_user]
}

pub const USER_PATH: &'static str = "/user/";



#[cfg(test)]
mod test {
    use super::super::super::rocket; // initialize the webserver
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn get_user() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/api/user/some_uuid_or_something").dispatch();
        assert_eq!(response.status(), Status::Ok);
//        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}
use rocket::Route;
use rocket_contrib::Json;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    password: String,
    id: String // Uuid
}

impl User {
    fn new(name: String, password: String ) -> User {
        User {
            name: name,
            password: password,
            id: Uuid::new_v4().hyphenated().to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct NewUser {
    name: String,
    password: String
}


#[get("/<user_id>")]
fn get_user(user_id: String) -> Json<User> {
    info!("Getting user with ID: {}", user_id);
    Json(User {
        name: String::from("test"),
        password: String::from("password"),
        id: user_id,
    })
}

#[post("/", data = "<new_user>")]
fn create_user(new_user: Json<NewUser>) -> Json<User> {
    info!("Creating new user with the following values: {:?}", new_user);
    let new_user: NewUser = new_user.into_inner();
    Json(User::new(new_user.name, new_user.password))
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
        password: String::from("password"),
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
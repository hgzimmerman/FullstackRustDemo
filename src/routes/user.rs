use rocket::Route;
use rocket_contrib::Json;

use routes::Routable;
use diesel::result::Error;
use chrono::{NaiveDateTime};
use db::Conn;

use db::user::User;
use db::user::NewUser;
use requests_and_responses::user::{NewUserRequest, UpdateDisplayNameRequest, UserResponse};
use rocket::response::status::Custom;
use rocket::http::Status;

// use routes::DatabaseError;
use routes::WeekendAtJoesError;



#[get("/<user_id>")]
fn get_user(user_id: i32, conn: Conn) -> Option<Json<UserResponse>> {
    User::get_user(user_id, &conn).and_then(|user|{
        let user_response: UserResponse = user.into();
        Some(Json(user_response))
    })
}



#[post("/", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUserRequest>, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {
    let new_user: NewUserRequest = new_user.into_inner();
    match User::create_user(new_user, &conn) {
        Ok(user) => {
            let user_response: UserResponse = user.into();
            Ok(Json(user_response))
        }
        Err(_) => {
            Err(WeekendAtJoesError::DatabaseError(None))
        }
    }
 
}


#[put("/", data = "<data>")]
fn update_user_display_name(data: Json<UpdateDisplayNameRequest>, conn: Conn ) -> Option<Json<UserResponse>> {

    let request: UpdateDisplayNameRequest = data.into_inner();
    let updated_user = User::update_user_display_name(request, &conn);

    match updated_user {
        Ok(updated_user) => {
            let user_response: UserResponse = updated_user.into();
            Some(Json(user_response))
        }
        Err(_) => None
    }
}


/// Currently, this is not exposed as an API, but is useful in testing
#[delete("/<user_id>")]
fn delete_user(user_id: i32, conn: Conn) -> Option<Json<UserResponse>> {

    let updated_user = User::delete_user_by_id(user_id, &conn);

    match updated_user {
        Ok(updated_user) => {
            let user_response: UserResponse = updated_user.into();
            Some(Json(user_response))
        }
        Err(_) => None
    }
}

#[delete("/<user_name>")]
pub fn delete_user_by_name(user_name: String, conn: Conn) -> Option<Json<UserResponse>> {

    let updated_user = User::delete_user_by_name(user_name, &conn);

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
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![create_user, update_user_display_name, get_user, delete_user, delete_user_by_name];
    const PATH: &'static str = "/user/";
}



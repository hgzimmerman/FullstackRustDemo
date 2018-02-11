use rocket::Route;
use rocket_contrib::Json;

use routes::Routable;
use diesel::result::Error;
use chrono::{NaiveDateTime};
use db::Conn;


use db::user::User;
use db::user::NewUser;
// use db::user::NewUserRequest;
// use db::user::UpdateDisplayNameRequest;
use requests_and_responses::user::{NewUserRequest, UpdateDisplayNameRequest, UserResponse};






#[get("/<user_id>")]
fn get_user(user_id: i32, conn: Conn) -> Option<Json<UserResponse>> {
    User::get_user(user_id, &conn).and_then(|user|{
        let user_response: UserResponse = user.into();
        Some(Json(user_response))
    })
    
    // use schema::users::dsl::*;
    // info!("Getting user with ID: {}", user_id);

    // let returned_users: Vec<User> = users
    //     .filter(id.eq(user_id))
    //     .limit(1)
    //     .load::<User>(&*conn)
    //     .expect("db error");

    // match returned_users.get(0) {
    //     Some(user) => {
    //         let user_response: UserResponse = user.clone().into();
    //         Some(Json(user_response))
    //     },
    //     None => None
    // }
}


use rocket::response::status::Custom;
use rocket::http::Status;


#[post("/", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUserRequest>, conn: Conn) -> Result<Json<UserResponse>, Custom<&'static str>> {
    // use schema::users;

    // info!("Creating new user with the following values: {:?}", new_user);
    // let new_user: NewUser = new_user.into_inner().into();

    // let inserted_user: User = diesel::insert_into(users::table)
    //     .values(&new_user)
    //     .get_result(&*conn)
    //     .expect("Couldn't create user");
   
    // let user_response: UserResponse = inserted_user.into();
    // Json(user_response)
    let new_user: NewUserRequest = new_user.into_inner();
    match User::create_user(new_user, &conn) {
        Ok(user) => {
            let user_response: UserResponse = user.into();
            Ok(Json(user_response))
        }
        Err(_) => {
            Err(Custom(Status::InternalServerError, "DB Error"))
        }
    }
 
}


#[put("/", data = "<data>")]
fn update_user_display_name(data: Json<UpdateDisplayNameRequest>, conn: Conn ) -> Option<Json<UserResponse>> {
    // use schema::users::dsl::*;
    // let data: UpdateDisplayNameRequest = data.into_inner();
    // info!("Updating the display name of user id {} to {}", data.id, data.new_display_name);

    // let target = users.filter(id.eq(data.id));

    // let updated_user: Result<User, Error> = diesel::update(target)
    //     .set(display_name.eq(data.new_display_name))
    //     .get_result(&*conn);
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

/// Currently, this is not exposed as an API, but is useful in testing
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
    const ROUTES: &'static Fn() -> Vec<Route> = &|| routes![create_user, update_user_display_name, get_user, delete_user];
    const PATH: &'static str = "/user/";
}



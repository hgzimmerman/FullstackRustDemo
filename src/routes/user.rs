use rocket::Route;
use rocket_contrib::Json;
use routes::{Routable, convert_vector};
use db::Conn;
use db::Deletable;
use db::Retrievable;
use db::Creatable;
use db::user::User;
use requests_and_responses::user::*;
use error::WeekendAtJoesError;
use auth::user_authorization::*;


#[get("/<user_id>")]
fn get_user(user_id: i32, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {
    User::get_by_id(user_id, &conn)
        .map(UserResponse::from)
        .map(Json)
}

// TODO Consider requiring admin access for this.
#[get("/users/<num_users>")]
fn get_users(num_users: i64, conn: Conn) -> Result<Json<Vec<UserResponse>>, WeekendAtJoesError> {
    User::get_users(num_users, &conn)
        .map(convert_vector)
        .map(Json)
}

#[get("/users_with_role/<role_id>")]
fn get_users_with_role(role_id: i32, conn: Conn) -> Result<Json<Vec<UserResponse>>, WeekendAtJoesError> {
    User::get_users_with_role(role_id.into(), &conn)
        .map(convert_vector)
        .map(Json)
}

use db::user::NewUser;

#[post("/", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUserRequest>, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {
    let new_user: NewUser = new_user.into_inner().into();
    User::create(new_user, &conn)
        .map(UserResponse::from)
        .map(Json)
}


#[put("/", data = "<data>")]
fn update_user_display_name(data: Json<UpdateDisplayNameRequest>, _user: NormalUser, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {
    info!("updating user display name");
    let request: UpdateDisplayNameRequest = data.into_inner();
    User::update_user_display_name(request, &conn)
        .map(UserResponse::from)
        .map(Json)
}

#[put("/assign_role", data = "<data>")]
fn assign_role(data: Json<UserRoleRequest>, _admin: AdminUser, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {

    User::add_role_to_user(data.id, data.user_role.into(), &conn)
        .map(UserResponse::from)
        .map(Json)
}


#[delete("/<user_id>")]
fn delete_user(user_id: i32, _admin: AdminUser, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {

    User::delete_by_id(user_id, &conn)
        .map(UserResponse::from)
        .map(Json)
}

#[delete("/<user_name>", rank = 2)]
pub fn delete_user_by_name(user_name: String, _admin: AdminUser, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {

    User::delete_user_by_name(user_name, &conn)
        .map(UserResponse::from)
        .map(Json)
}

// Export the ROUTES and their path
impl Routable for User {
    const ROUTES: &'static Fn() -> Vec<Route> = &|| {
        routes![
            create_user,
            update_user_display_name,
            get_user,
            get_users,
            get_users_with_role,
            assign_role,
            delete_user,
            delete_user_by_name,
        ]
    };
    const PATH: &'static str = "/user/";
}

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
use db::user::NewUser;

/// Gets basic info about an user.
/// Provided they know the id of the user, this information is available to anyone.
#[get("/<user_id>")]
fn get_user(user_id: i32, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {
    User::get_by_id(user_id, &conn)
        .map(UserResponse::from)
        .map(Json)
}

/// Get all info about users.
/// This request is paginated. It will return vectors of users, 25 at a time.
/// This operation is only available to an admin.
#[get("/users/<index>")]
fn get_users(index: i32, _admin: AdminUser, conn: Conn) -> Result<Json<Vec<FullUserResponse>>, WeekendAtJoesError> {
    User::get_paginated(index, 25, &conn)
        .map(|x| x.0)
        .map(convert_vector)
        .map(Json)
}

/// Get all users with the specified role id.
/// This operation is only available to an admin.
#[get("/users_with_role/<role_id>")]
fn get_users_with_role(role_id: i32, _admin: AdminUser, conn: Conn) -> Result<Json<Vec<UserResponse>>, WeekendAtJoesError> {
    User::get_users_with_role(role_id.into(), &conn)
        .map(convert_vector)
        .map(Json)
}

/// It doesn't require any account to create a new user.
#[post("/", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUserRequest>, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {
    let new_user: NewUser = new_user.into_inner().into();
    User::create(new_user, &conn)
        .map(UserResponse::from)
        .map(Json)
}


/// Allow a user to update their display name.
#[put("/", data = "<data>")]
fn update_user_display_name(data: Json<UpdateDisplayNameRequest>, _user: NormalUser, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {
    info!("updating user display name");
    let request: UpdateDisplayNameRequest = data.into_inner();
    // TODO, check if this is valid.
    User::update_user_display_name(request, &conn)
        .map(UserResponse::from)
        .map(Json)
}

/// Assigns a role to a user.
/// This operation is only available to an admin.
#[put("/assign_role", data = "<data>")]
fn assign_role(data: Json<UserRoleRequest>, _admin: AdminUser, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {

    User::add_role_to_user(data.id, data.user_role.into(), &conn)
        .map(UserResponse::from)
        .map(Json)
}


// I'm not sure if deleting users is a good option. I don't want to orphan their data.
// TODO consider banning instead?
#[delete("/<user_id>")]
fn delete_user(user_id: i32, _admin: AdminUser, conn: Conn) -> Result<Json<UserResponse>, WeekendAtJoesError> {

    User::delete_by_id(user_id, &conn)
        .map(UserResponse::from)
        .map(Json)
}

// I'm not sure if deleting users is a good option. I don't want to orphan their data.
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

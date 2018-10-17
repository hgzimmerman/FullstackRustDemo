use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use wire::user::UserResponse;
use identifiers::user::UserUuid;
use uuid::Uuid;
use crate::db_integration::db_filter;
use crate::uuid_integration::uuid_filter;
use db::Conn;
use db::user::User;
use db::RetrievableUuid;

use crate::jwt::jwt_filter;
use crate::error::Error;
use crate::jwt::admin_user_filter;
//use wire::convert_vector;
use wire::user::FullUserResponse;
use wire::user::NewUserRequest;
use db::user::NewUser;
use db::CreatableUuid;
use crate::jwt::normal_user_filter;
use wire::user::UpdateDisplayNameRequest;
use wire::user::UserRoleRequest;
//use crate::HttpMethod;
//use crate::log_attach;
use crate::logging::log_attach;
use crate::logging::HttpMethod;
use crate::util::convert_and_json;
use crate::util::convert_vector_and_json;
use crate::uuid_integration::uuid_wrap_filter;

pub fn user_api() -> BoxedFilter<(impl warp::Reply,)> {
    info!("Attaching User API");
    warp::path("user")
        .and(
            simple_filter()
                .or(get_user())
                .or(get_users())
                .or(create_user())
                .or(update_user_display_name())
                .or(add_role())
                .or(ban_user())
                .or(unban_user())
        )
        .with(warp::log("user"))
        .boxed()
}

pub fn simple_filter() -> BoxedFilter<(impl Reply,)> {
    fn user_response() -> UserResponse {
        UserResponse {
            user_name: String::new(),
            display_name: String::new(),
            uuid: UserUuid(Uuid::default()),
        }
    }
    let u_response = warp::any().map(move || user_response());

    return warp::get2()
        .and(warp::path("simple"))
        .and(warp::path::index())
        .and(u_response)
        .map( |user| warp::reply::json(&user) )
        .boxed()
}

pub fn boxed_filter_that_relies_on_db() -> BoxedFilter<(impl Reply,)>{
    warp::get2()
        .and(warp::path("yeet"))
        .and(db_filter())
        .map(|_conn: Conn| "hello there")
        .boxed()
}

pub fn jwt_test() -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path("jwt"))
        .and(jwt_filter())
        .map(|_| "works")
        .boxed()
}

// ==== Actual User API =====

fn get_user() -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "user/");

    warp::get2()
        .and(uuid_filter())
        .and(db_filter())
        .and_then(|user_uuid: Uuid, conn: Conn| {
            User::get_by_uuid(user_uuid, &conn)
                .map(convert_and_json::<User, UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn get_users() -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "user/<i32>");

    warp::get2()
        .and(warp::path::param::<i32>())
        .and(admin_user_filter())
        .and(db_filter())
        .and_then(|index: i32, _admin: UserUuid, conn: Conn| {
            User::get_paginated(index, 25, &conn)
                .map(|x:(Vec<User>,i64)| x.0)
                .map(convert_vector_and_json::<User,FullUserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn create_user() -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Post, "user/");

    let json_body = warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json());
    warp::post2()
        .and(json_body)
        .and(admin_user_filter())
        .and(db_filter())
        .and_then(|new_user: NewUserRequest, _admin: UserUuid, conn: Conn|{
                let new_user: NewUser = new_user.into();
                User::create(new_user, &conn)
                    .map(convert_and_json::<User,UserResponse>)
                    .map_err(Error::convert_and_reject)
        })
        .boxed()
}

// TODO because this API doesn't rely on the UUID from the auth header, it can be spoofed to alter another user's display nome.
// It currently is the way it is for maintaining consistency with the old implementation. but it should be changed.
fn update_user_display_name() -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "user/display_name");
//    use crate::log_attach_in_out;
//    log_attach_in_out::<UpdateDisplayNameRequest, UserResponse>(HttpMethod::Put, "user/display_name");
    let json_body = warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json());
    warp::put2()
        .and(warp::path("display_name"))
        .and(json_body)
        .and(normal_user_filter())
        .and(db_filter())
        .and_then(|request: UpdateDisplayNameRequest, user_uuid: UserUuid, conn: Conn| {
            let new_display_name = request.new_display_name;
            User::update_user_display_name_safe(user_uuid, new_display_name, &conn)
                .map(convert_and_json::<User,UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn add_role() -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "user/assign_role");

    let json_body = warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json());
    warp::put2()
        .and(warp::path("assign_role"))
        .and(json_body)
        .and(admin_user_filter())
        .and(db_filter())
        .and_then(|request: UserRoleRequest, _user: UserUuid, conn: Conn| {
            User::add_role_to_user(request.uuid, request.user_role.into(), &conn)
                .map(convert_and_json::<User,UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

// TODO technically incomplete because it fails to prevent active JWTs from authenticating.
// Needs to get an rwlock to lock around some struct to prevent this like in the Rocket server.
fn ban_user() -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Put, "user/ban/<uuid>");
    warp::put2()
        .and(path!("ban"))
        .and(uuid_wrap_filter::<UserUuid>())
        .and(admin_user_filter())
        .and(db_filter())
        .and_then(|user_uuid: UserUuid, _user: UserUuid, conn: Conn| {
            User::set_ban_status(user_uuid, true, &conn)
                .map(convert_and_json::<User,UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn unban_user() -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "user/unban/<uuid>");

    warp::put2()
        .and(warp::path("unban"))
        .and(uuid_wrap_filter::<UserUuid>())
        .and(admin_user_filter())
        .and(db_filter())
        .and_then(|user_uuid: UserUuid, _user: UserUuid, conn: Conn| {
            User::set_ban_status(user_uuid, false, &conn)
                .map(convert_and_json::<User,UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}


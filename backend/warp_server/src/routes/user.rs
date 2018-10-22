use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;
use wire::user::UserResponse;
use identifiers::user::UserUuid;
use uuid::Uuid;
//use crate::db_integration::s.db.clone();
use crate::uuid_integration::uuid_filter;
use db::Conn;
use db::user::User;
use db::RetrievableUuid;

//use crate::jwt::jwt_filter;
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
use crate::state::State;
use pool::PooledConn;

pub fn user_api(s: &State) -> BoxedFilter<(impl warp::Reply,)> {
    info!("Attaching User API");
    warp::path("user")
        .and(
            get_user(s)
                .or(get_users(s))
                .or(create_user(s))
                .or(update_user_display_name(s))
                .or(add_role(s))
                .or(ban_user(s))
                .or(unban_user(s))
        )
        .with(warp::log("user"))
        .boxed()
}



fn get_user(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "user/<uuid>");

    warp::get2()
        .and(uuid_filter())
        .and(s.db.clone())
        .and_then(|user_uuid: Uuid, conn: PooledConn| {
            User::get_by_uuid(user_uuid, &conn)
                .map(convert_and_json::<User, UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn get_users(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Get, "user/<i32>");

    warp::get2()
        .and(warp::path::param::<i32>())
        .and(admin_user_filter(s))
        .and(s.db.clone())
        .and_then(|index: i32, _admin: UserUuid, conn: PooledConn| {
            User::get_paginated(index, 25, &conn)
                .map(|x:(Vec<User>,i64)| x.0)
                .map(convert_vector_and_json::<User,FullUserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn create_user(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Post, "user/");

    let json_body = warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json());
    warp::post2()
        .and(json_body)
        .and(admin_user_filter(s))
        .and(s.db.clone())
        .and_then(|new_user: NewUserRequest, _admin: UserUuid, conn: PooledConn|{
                let new_user: NewUser = new_user.into();
                User::create(new_user, &conn)
                    .map(convert_and_json::<User,UserResponse>)
                    .map_err(Error::convert_and_reject)
        })
        .boxed()
}

// TODO because this API doesn't rely on the UUID from the auth header, it can be spoofed to alter another user's display nome.
// It currently is the way it is for maintaining consistency with the old implementation. but it should be changed.
fn update_user_display_name(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "user/display_name");
//    use crate::log_attach_in_out;
//    log_attach_in_out::<UpdateDisplayNameRequest, UserResponse>(HttpMethod::Put, "user/display_name");
    let json_body = warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json());
    warp::put2()
        .and(warp::path("display_name"))
        .and(json_body)
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: UpdateDisplayNameRequest, user_uuid: UserUuid, conn: PooledConn| {
            let new_display_name = request.new_display_name;
            User::update_user_display_name_safe(user_uuid, new_display_name, &conn)
                .map(convert_and_json::<User,UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn add_role(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "user/assign_role");

    let json_body = warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json());
    warp::put2()
        .and(warp::path("assign_role"))
        .and(json_body)
        .and(admin_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: UserRoleRequest, _user: UserUuid, conn: PooledConn| {
            User::add_role_to_user(request.uuid, request.user_role.into(), &conn)
                .map(convert_and_json::<User,UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

// TODO technically incomplete because it fails to prevent active JWTs from authenticating.
// Needs to get an rwlock to lock around some struct to prevent this like in the Rocket server.
fn ban_user(s: &State) -> BoxedFilter<(impl Reply,)> {

    log_attach(HttpMethod::Put, "user/ban/<uuid>");
    warp::put2()
        .and(path!("ban"))
        .and(uuid_wrap_filter::<UserUuid>())
        .and(admin_user_filter(s))
        .and(s.db.clone())
        .and_then(|user_uuid: UserUuid, _user: UserUuid, conn: PooledConn| {
            User::set_ban_status(user_uuid, true, &conn)
                .map(convert_and_json::<User,UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

fn unban_user(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "user/unban/<uuid>");

    warp::put2()
        .and(warp::path("unban"))
        .and(uuid_wrap_filter::<UserUuid>())
        .and(admin_user_filter(s))
        .and(s.db.clone())
        .and_then(|user_uuid: UserUuid, _user: UserUuid, conn: PooledConn| {
            User::set_ban_status(user_uuid, false, &conn)
                .map(convert_and_json::<User,UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use testing_fixtures::fixtures::user::UserFixture;
    use testing_common::setup::setup_warp;
    use pool::Pool;
    use crate::util::test::deserialize;

    #[test]
    fn get() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let s = State::testing_init(pool, fixture.secret.clone());
            let response = warp::test::request()
                .method("GET")
                .path(&format!("/user/{}", fixture.normal_user.uuid))
                .reply(&user_api(&s));

            assert_eq!(response.status(), 200);
            let user: UserResponse = deserialize(response);

            assert_eq!(user.display_name, fixture.normal_user.display_name);
        })
    }
}
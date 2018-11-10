use db::user::{
    NewUser,
    User,
};
use identifiers::user::UserUuid;
use warp::{
    filters::BoxedFilter,
    reply::Reply,
    Filter,
};
use wire::user::{
    FullUserResponse,
    NewUserRequest,
    UpdateDisplayNameRequest,
    UserResponse,
    UserRoleRequest,
};

use crate::{
    logging::{
        log_attach,
        HttpMethod,
    },
    state::{
        banned_list::BannedList,
        jwt::{
            admin_user_filter,
            normal_user_filter,
        },
        State,
    },
    util::{
        convert_and_json,
        convert_vector_and_json,
    },
    uuid_integration::uuid_wrap_filter,
};
use error::Error;
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
                .or(unban_user(s)),
        )
        .with(warp::log("user"))
        .boxed()
}

fn get_user(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "user/<uuid>");

    warp::get2()
        .and(uuid_wrap_filter())
        .and(s.db.clone())
        .and_then(|user_uuid: UserUuid, conn: PooledConn| {
            User::get_user(user_uuid, &conn)
                .map(convert_and_json::<User, UserResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

fn get_users(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "user/<i32 where i32 >= 1>");

    warp::get2()
        .and(warp::path::param::<i32>())
        .and(admin_user_filter(s))
        .and(s.db.clone())
        .and_then(|index: i32, _admin: UserUuid, conn: PooledConn| {
            User::get_paginated(index, 25, &conn)
                .map(|x: (Vec<User>, i64)| x.0)
                .map(convert_vector_and_json::<User, FullUserResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

fn create_user(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Post, "user/");

    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
    warp::post2()
        .and(warp::path::end())
        .and(json_body)
        //        .and(admin_user_filter(s))
        .and(s.db.clone())
        .and_then(|new_user: NewUserRequest, conn: PooledConn| {
            let new_user: NewUser = new_user.into();
            User::create_user(new_user, &conn)
                .map(convert_and_json::<User, UserResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

// TODO because this API doesn't rely on the UUID from the auth header, it can be spoofed to alter another user's display nome.
// It currently is the way it is for maintaining consistency with the old implementation. but it should be changed.
fn update_user_display_name(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "user/display_name");
    //    use crate::log_attach_in_out;
    //    log_attach_in_out::<UpdateDisplayNameRequest, UserResponse>(HttpMethod::Put, "user/display_name");
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
    warp::put2()
        .and(warp::path("display_name"))
        .and(json_body)
        .and(normal_user_filter(s))
        .and(s.db.clone())
        .and_then(
            |request: UpdateDisplayNameRequest, user_uuid: UserUuid, conn: PooledConn| {
                let new_display_name = request.new_display_name;
                User::update_user_display_name_safe(user_uuid, new_display_name, &conn)
                    .map(convert_and_json::<User, UserResponse>)
                    .map_err(Error::simple_reject)
            },
        )
        .boxed()
}

fn add_role(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "user/assign_role");

    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());
    warp::put2()
        .and(warp::path("assign_role"))
        .and(json_body)
        .and(admin_user_filter(s))
        .and(s.db.clone())
        .and_then(|request: UserRoleRequest, _user: UserUuid, conn: PooledConn| {
            User::add_role_to_user(request.uuid, request.user_role.into(), &conn)
                .map(convert_and_json::<User, UserResponse>)
                .map_err(Error::simple_reject)
        })
        .boxed()
}

fn ban_user(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "user/ban/<uuid>");
    warp::put2()
        .and(path!("ban"))
        .and(uuid_wrap_filter::<UserUuid>())
        .and(admin_user_filter(s))
        .and(s.banned_list.clone())
        .and(s.db.clone())
        .and_then(
            |user_uuid: UserUuid, user: UserUuid, banned_list: BannedList, conn: PooledConn| {
                let resp = User::set_ban_status(user_uuid, true, &conn)
                    .map(convert_and_json::<User, UserResponse>)
                    .map_err(Error::simple_reject);
                banned_list.ban(user);
                resp
            },
        )
        .boxed()
}

fn unban_user(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Put, "user/unban/<uuid>");

    warp::put2()
        .and(warp::path("unban"))
        .and(uuid_wrap_filter::<UserUuid>())
        .and(admin_user_filter(s))
        .and(s.banned_list.clone())
        .and(s.db.clone())
        .and_then(
            |user_uuid: UserUuid, user: UserUuid, banned_list: BannedList, conn: PooledConn| {
                let resp = User::set_ban_status(user_uuid, false, &conn)
                    .map(convert_and_json::<User, UserResponse>)
                    .map_err(Error::simple_reject);
                banned_list.unban(&user);
                resp
            },
        )
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        state::jwt::AUTHORIZATION_HEADER_KEY,
        util::test::deserialize,
    };
    use pool::Pool;
    use testing_common::setup::setup_warp;
    use testing_fixtures::fixtures::user::UserFixture;
    use wire::user::{
        UserRole,
        BEARER,
    };

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

    #[test]
    fn get_many() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let s = State::testing_init(pool, fixture.secret.clone());
            let jwt: String = crate::routes::auth::tests::get_admin_jwt_string(&s, fixture);
            let response = warp::test::request()
                .method("GET")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path("/user/1") // get the first page
                .reply(&user_api(&s));

            assert_eq!(response.status(), 200);
            let users: Vec<FullUserResponse> = deserialize(response);

            assert_eq!(users.len(), 2)
        })
    }

    #[test]
    fn create() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let s = State::testing_init(pool, fixture.secret.clone());
            let request = NewUserRequest {
                user_name: String::from("user name"),
                display_name: String::from("display name"),
                plaintext_password: String::from("password_aoeuaoeu"),
            };
            let response = warp::test::request()
                .method("POST")
                .header("Content-Length", "1000") // Requires sized length
                .json(&request)
                .path("/user")
                .reply(&user_api(&s));

            assert_eq!(response.status(), 200);
            let user: UserResponse = deserialize(response);
            assert_eq!(user.user_name, String::from("user name"))
        })
    }

    #[test]
    fn update_user_display_name() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let s = State::testing_init(pool, fixture.secret.clone());

            let user_name = fixture.normal_user.user_name.clone();
            let request = UpdateDisplayNameRequest {
                user_name: user_name.clone(), // TODO, this is bad API design, this should instead use the JWT's user ID to use as a key.
                new_display_name: String::from("yeet"),
            };
            let jwt: String = crate::routes::auth::tests::get_jwt_string(&s, user_name);

            let response = warp::test::request()
                .method("PUT")
                .header("Content-Length", "1000") // Requires sized length
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .json(&request)
                .path("/user/display_name")
                .reply(&user_api(&s));

            assert_eq!(response.status(), 200);
            let user: UserResponse = deserialize(response);
            assert_eq!(user.display_name, String::from("yeet"))
        })
    }

    #[test]
    fn add_role() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let s = State::testing_init(pool, fixture.secret.clone());

            let user_name = fixture.admin_user.user_name.clone();
            let request = UserRoleRequest {
                uuid: UserUuid(fixture.normal_user.uuid),
                user_role: UserRole::Admin.into(),
            };
            let jwt: String = crate::routes::auth::tests::get_jwt_string(&s, user_name);

            let response = warp::test::request()
                .method("PUT")
                .header("Content-Length", "1000") // Requires sized length
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .json(&request)
                .path("/user/assign_role")
                .reply(&user_api(&s));

            assert_eq!(response.status(), 200);
            let _user: UserResponse = deserialize(response);
        })
    }

    #[test]
    fn ban() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let s = State::testing_init(pool, fixture.secret.clone());

            let admin_name = fixture.admin_user.user_name.clone();
            let request = UserRoleRequest {
                uuid: UserUuid(fixture.normal_user.uuid),
                user_role: UserRole::Admin.into(),
            };
            let jwt: String = crate::routes::auth::tests::get_jwt_string(&s, admin_name);

            let response = warp::test::request()
                .method("PUT")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path(&format!("/user/ban/{}", fixture.normal_user.uuid.clone()))
                .reply(&user_api(&s));

            assert_eq!(response.status(), 200);
            let _user: UserResponse = deserialize(response);
        })
    }

    #[test]
    fn unban() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let s = State::testing_init(pool, fixture.secret.clone());

            let admin_name = fixture.admin_user.user_name.clone();
            let request = UserRoleRequest {
                uuid: UserUuid(fixture.normal_user.uuid),
                user_role: UserRole::Admin.into(),
            };
            let jwt: String = crate::routes::auth::tests::get_jwt_string(&s, admin_name);

            let response = warp::test::request()
                .method("PUT")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt))
                .path(&format!("/user/unban/{}", fixture.normal_user.uuid.clone()))
                .reply(&user_api(&s));

            assert_eq!(response.status(), 200);
            let _user: UserResponse = deserialize(response);
        })
    }

}

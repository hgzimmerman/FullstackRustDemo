use warp::{
    self,
    filters::BoxedFilter,
    reject::Rejection,
    reply::Reply,
    Filter,
};

use auth::{
    Secret,
    ServerJwt,
};
use crate::{
    error::Error,
    logging::{
        log_attach,
        HttpMethod,
    },
    state::{
        jwt::jwt_filter,
        State,
    },
};
use db::auth as auth_db;
use pool::PooledConn;
use wire::login::LoginRequest;

pub fn auth_api(s: &State) -> BoxedFilter<(impl warp::Reply,)> {
    info!("Attaching Auth API");
    warp::path("auth")
        .and(reauth(s).or(login(s)))
        .with(warp::log("auth"))
        .boxed()
}

fn reauth(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Get, "auth/reauth");
    warp::get2()
        .and(warp::path("reauth"))
        .and(s.secret.clone())
        .and(jwt_filter(s))
        .and_then(|secret: Secret, jwt: ServerJwt| {
            auth_db::reauth(jwt, &secret).map_err(|_| {
                Error::NotAuthorized {
                    reason: "Could not reauth",
                }
                .simple_reject()
            })
        })
        .boxed()
}

fn login(s: &State) -> BoxedFilter<(impl Reply,)> {
    log_attach(HttpMethod::Post, "auth/login");

    fn handle_login(secret: Secret, conn: PooledConn, login_request: LoginRequest) -> Result<impl Reply, Rejection> {
        auth_db::login(login_request, &secret, &conn).map_err(|_| {
            Error::NotAuthorized {
                reason: "Invalid password or username",
            }
            .simple_reject()
        }) // TODO I can do better than this error message.
    }

    warp::post2()
        .and(warp::path("login"))
        .and(s.secret.clone())
        .and(s.db.clone())
        .and(warp::body::json())
        .and_then(handle_login)
        .boxed()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        state::jwt::AUTHORIZATION_HEADER_KEY,
        util::test::deserialize_string,
    };
    use pool::Pool;
    use serde_json::to_string as serde_ser;
    use testing_common::setup::setup_warp;
    use testing_fixtures::fixtures::user::UserFixture;
    use wire::user::BEARER;

    /// Utility for getting the jwt string.
    /// This should make
    pub fn get_admin_jwt_string(s: &State, fixture: &UserFixture) -> String {
        let request = LoginRequest {
            user_name: fixture.admin_user.user_name.clone(),
            password: String::from(testing_fixtures::fixtures::user::PASSWORD),
        };
        let response = warp::test::request()
            .method("POST")
            .json(&request)
            .path("/auth/login")
            .reply(&auth_api(&s));
        let jwt_string: String = deserialize_string(response);
        jwt_string
    }

    pub fn get_jwt_string(s: &State, user_name: String) -> String {
        let request = LoginRequest {
            user_name,
            password: String::from(testing_fixtures::fixtures::user::PASSWORD),
        };
        let response = warp::test::request()
            .method("POST")
            .json(&request)
            .path("/auth/login")
            .reply(&auth_api(&s));
        let jwt_string: String = deserialize_string(response);
        jwt_string
    }

    #[test]
    fn end_to_end_auth() {
        setup_warp(|fixture: &UserFixture, pool: Pool| {
            let s = State::testing_init(pool, fixture.secret.clone());
            let request = LoginRequest {
                user_name: fixture.admin_user.user_name.clone(),
                password: String::from(testing_fixtures::fixtures::user::PASSWORD),
            };
            let response = warp::test::request()
                .method("POST")
                .json(&request)
                .path("/auth/login")
                .reply(&auth_api(&s));

            assert_eq!(response.status(), 200);
            let jwt_string: String = deserialize_string(response);

            let response = warp::test::request()
                .method("GET")
                .header(AUTHORIZATION_HEADER_KEY, format!("{} {}", BEARER, jwt_string).as_str())
                .path("/auth/reauth")
                .reply(&auth_api(&s));

            assert_eq!(response.status(), 200);
            let new_jwt_string: String = deserialize_string(response);

            assert_ne!(new_jwt_string, jwt_string);
        })
    }
}

use warp::filters::BoxedFilter;
use warp;
//use wire::user::Jwt;
use wire::user::BEARER;
use auth::ServerJwt;
use std::result::Result::Err;
use warp::Filter;
use auth::Secret;
use std::sync::RwLock;
use wire::user::UserRole;
use identifiers::user::UserUuid;


use crate::error::Error;


pub fn jwt_filter() -> BoxedFilter<(ServerJwt,)> {
    warp::header::header::<String>("Authorization")
        .or_else(|_| Error::MalformedToken.reject())
        .and(secret_filter())
        .and_then(|bearer_string: String, secret: Secret| {
            extract_jwt(bearer_string, &secret)
                .map_err(|e: Error|warp::reject().with(e))
        })
        .boxed()
}

pub fn secret_filter() -> BoxedFilter<(Secret,)> {
    warp::any()
        .map(|| get_secret())
        .boxed()
}

#[allow(dead_code)]
pub fn admin_user_filter() -> BoxedFilter<(UserUuid,)> {
    warp::any()
        .and(jwt_filter())
        .and_then(|server_jwt: ServerJwt| {
            if server_jwt.0.user_roles.contains(&UserRole::Admin) {
                return Ok(server_jwt.0.sub)
            } else {
                Error::NotAuthorized.reject()
            }
        })
        .boxed()
}

#[allow(dead_code)]
pub fn normal_user_filter() -> BoxedFilter<(UserUuid,)> {
    warp::any()
        .and(jwt_filter())
        .and_then(|server_jwt: ServerJwt| {
            if server_jwt.0.user_roles.contains(&UserRole::Unprivileged) {
                return Ok(server_jwt.0.sub)
            } else {
                Error::NotAuthorized.reject()
            }
        })
        .boxed()
}

#[allow(dead_code)]
pub fn publisher_user_filter() -> BoxedFilter<(UserUuid,)> {
    warp::any()
        .and(jwt_filter())
        .and_then(|server_jwt: ServerJwt| {
            if server_jwt.0.user_roles.contains(&UserRole::Publisher) {
                return Ok(server_jwt.0.sub)
            } else {
                Error::NotAuthorized.reject()
            }
        })
        .boxed()
}
#[allow(dead_code)]
pub fn moderator_user_filter() -> BoxedFilter<(UserUuid,)> {
    warp::any()
        .and(jwt_filter())
        .and_then(|server_jwt: ServerJwt| {
            if server_jwt.0.user_roles.contains(&UserRole::Moderator) {
                return Ok(server_jwt.0.sub)
            } else {
                Error::NotAuthorized.reject()
            }
        })
        .boxed()
}

fn extract_jwt(bearer_string: String, secret: &Secret) -> Result<ServerJwt, Error>{
    let authorization_words: Vec<String> = bearer_string
        .split_whitespace()
        .map(String::from)
        .collect();

    if authorization_words.len() != 2 {
        return Err(Error::MalformedToken)
    }
    if authorization_words[0] != BEARER {
        return Err(Error::MalformedToken)
    }
    let jwt_str: &str = &authorization_words[1];

    ServerJwt::decode_jwt_string(jwt_str, secret).map_err(|_| Error::IllegalToken)

}


fn get_secret() -> Secret {
    SECRET.read().unwrap().clone()
}

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref SECRET: RwLock<Secret> = RwLock::new(Secret::generate());
}
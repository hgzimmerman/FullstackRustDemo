use warp::{
    self,
    filters::BoxedFilter,
    Filter,
    reject::Rejection
};
use wire::user::BEARER;
use auth::{
    ServerJwt,
    Secret
};
use std::result::Result::Err;
use wire::user::UserRole;
use identifiers::user::UserUuid;


//use crate::error::Error;
use crate::state::State;
use crate::state::banned_list::BannedList;
use error::Error;

pub const AUTHORIZATION_HEADER_KEY: &str = "Authorization";

/// Gets a JWT from the headers, decodes it to determine its authenticity, and then checks if its associated user is banned.
pub fn jwt_filter(s: &State) -> BoxedFilter<(ServerJwt,)> {
    /// Helper fn
    fn handle_jwt_extraction_and_verification(bearer_string: String, secret: Secret, banned_list: BannedList) -> Result<ServerJwt, Rejection> {
        let jwt = extract_jwt(bearer_string, &secret)
            .map_err(Error::simple_reject);

        // Check if the user is banned, and therefore their jwt should be rejected.
        if let Ok(ref jwt) = &jwt {
            if banned_list.is_banned(&jwt.0.sub) {
                return Error::UserBanned.reject()
            }
        }
        jwt

    }

    warp::header::header::<String>(AUTHORIZATION_HEADER_KEY)
        .or_else(|_| Error::MalformedToken.reject())
        .and(s.secret.clone())
        .and(s.banned_list.clone())
        .and_then(handle_jwt_extraction_and_verification)
        .boxed()
}

pub fn secret_filter(locked_secret: Secret) -> BoxedFilter<(Secret,)> {
    warp::any()
        .map(move || locked_secret.clone())
        .boxed()
}

#[allow(dead_code)]
pub fn admin_user_filter(s: &State) -> BoxedFilter<(UserUuid,)> {
    warp::any()
        .and(jwt_filter(s))
        .and_then(|server_jwt: ServerJwt| {
            if server_jwt.0.user_roles.contains(&UserRole::Admin) {
                return Ok(server_jwt.0.sub)
            } else {
                Error::NotAuthorized{reason: "JWT does not contain Admin privilege"}.reject()
            }
        })
        .boxed()
}

#[allow(dead_code)]
pub fn normal_user_filter(s: &State) -> BoxedFilter<(UserUuid,)> {
    warp::any()
        .and(jwt_filter(s))
        .and_then(|server_jwt: ServerJwt| {
            if server_jwt.0.user_roles.contains(&UserRole::Unprivileged) {
                return Ok(server_jwt.0.sub)
            } else {

                Error::NotAuthorized{reason: "JWT does not contain Basic User privilege"}.reject()
            }
        })
        .boxed()
}

/// Gets an Option<UserUuid> from the request.
pub fn optional_normal_user_filter(s: &State) -> BoxedFilter<(Option<UserUuid>,)> {

    fn handle_jwt(server_jwt: ServerJwt) -> Result<Option<UserUuid>, Rejection>{
         if server_jwt.0.user_roles.contains(&UserRole::Unprivileged) {
            return Ok(Some(server_jwt.0.sub))
        } else {
             return Ok(None)
        }
    }
    warp::any()
        .and(jwt_filter(s))
        .and_then(handle_jwt)
        .or(warp::any().map(||None))
        .unify::<(Option<UserUuid>,)>()
        .boxed()
}


#[allow(dead_code)]
pub fn publisher_user_filter(s: &State) -> BoxedFilter<(UserUuid,)> {
    warp::any()
        .and(jwt_filter(s))
        .and_then(|server_jwt: ServerJwt| {
            if server_jwt.0.user_roles.contains(&UserRole::Publisher) {
                return Ok(server_jwt.0.sub)
            } else {
                Error::NotAuthorized{reason: "JWT does not contain Publisher privilege"}.reject()
            }
        })
        .boxed()
}
#[allow(dead_code)]
pub fn moderator_user_filter(s: &State) -> BoxedFilter<(UserUuid,)> {
    warp::any()
        .and(jwt_filter(s))
        .and_then(|server_jwt: ServerJwt| {
            if server_jwt.0.user_roles.contains(&UserRole::Moderator) {
                return Ok(server_jwt.0.sub)
            } else {
                Error::NotAuthorized{reason: "JWT does not contain Moderator privilege"}.reject()
            }
        })
        .boxed()
}

/// Removes the jwt from the bearer string, and decodes it to determine if it was signed properly.
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

//
//fn get_secret() -> Secret {
//    SECRET.read().unwrap().clone()
//}
//
//lazy_static! {
//    /// This is an example for using doc comment attributes
//    static ref SECRET: RwLock<Secret> = RwLock::new(Secret::generate());
//}
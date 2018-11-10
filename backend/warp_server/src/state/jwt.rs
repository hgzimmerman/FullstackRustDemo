use auth::{
    Secret,
    ServerJwt,
};
use identifiers::user::UserUuid;
use std::result::Result::Err;
use warp::{
    self,
    filters::BoxedFilter,
    reject::Rejection,
    Filter,
};
use wire::user::{
    UserRole,
    BEARER,
};

//use crate::error::Error;
use crate::state::{
    banned_list::BannedList,
    State,
};
use error::Error;

pub const AUTHORIZATION_HEADER_KEY: &str = "Authorization";

/// Gets a JWT from the headers, decodes it to determine its authenticity, and then checks if its associated user is banned.
pub fn jwt_filter(s: &State) -> BoxedFilter<(ServerJwt,)> {
    /// Helper fn
    fn handle_jwt_extraction_and_verification(
        bearer_string: String,
        secret: Secret,
        banned_list: BannedList,
    ) -> Result<ServerJwt, Rejection> {
        let jwt = extract_jwt(bearer_string, &secret).map_err(Error::simple_reject);

        // Check if the user is banned, and therefore their jwt should be rejected.
        if let Ok(ref jwt) = &jwt {
            if banned_list.is_banned(&jwt.0.sub) {
                return Error::UserBanned.reject();
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

/// Brings the secret into scope.
pub fn secret_filter(locked_secret: Secret) -> BoxedFilter<(Secret,)> {
    warp::any().map(move || locked_secret.clone()).boxed()
}

/// Extract the uuid from the JWT.
fn get_user_uuid_from_jwt(server_jwt: ServerJwt) -> Result<UserUuid, Rejection> {
    return Ok(server_jwt.0.sub);
}

#[allow(dead_code)]
pub fn admin_user_filter(s: &State) -> BoxedFilter<(UserUuid,)> {
    warp::any()
        .and(jwt_filter(s))
        .and_then(|server_jwt: ServerJwt| {
            if server_jwt.0.user_roles.contains(&UserRole::Admin) {
                return get_user_uuid_from_jwt(server_jwt)
            } else {
                Error::NotAuthorized {
                    reason: "JWT does not contain Admin privilege",
                }
                .reject()
            }
        })
        .boxed()
}

/// If the user has a JWT, then the user has basic user privileges.
#[allow(dead_code)]
pub fn normal_user_filter(s: &State) -> BoxedFilter<(UserUuid,), > {
    warp::any()
        .and(jwt_filter(s))
        .and_then( get_user_uuid_from_jwt)
        .boxed()
}



/// Gets an Option<UserUuid> from the request.
/// Returns Some(user_uuid) if the user has a valid JWT, and None otherwise.
pub fn optional_normal_user_filter(s: &State) -> BoxedFilter<(Option<UserUuid>,)> {
    normal_user_filter(s)
        .map(Some)
        .or(warp::any().map(|| None))
        .unify::<(Option<UserUuid>,)>()
        .boxed()
}


#[allow(dead_code)]
pub fn publisher_user_filter(s: &State) -> BoxedFilter<(UserUuid,)> {
    warp::any()
        .and(jwt_filter(s))
        .and_then(|server_jwt: ServerJwt| {
            if server_jwt.0.user_roles.contains(&UserRole::Publisher) {
                return get_user_uuid_from_jwt(server_jwt)
            } else {
                Error::NotAuthorized {
                    reason: "JWT does not contain Publisher privilege",
                }
                .reject()
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
                return get_user_uuid_from_jwt(server_jwt)
            } else {
                Error::NotAuthorized {
                    reason: "JWT does not contain Moderator privilege",
                }
                .reject()
            }
        })
        .boxed()
}

/// Removes the jwt from the bearer string, and decodes it to determine if it was signed properly.
fn extract_jwt(bearer_string: String, secret: &Secret) -> Result<ServerJwt, Error> {
    let authorization_words: Vec<String> = bearer_string.split_whitespace().map(String::from).collect();

    if authorization_words.len() != 2 {
        return Err(Error::MalformedToken);
    }
    if authorization_words[0] != BEARER {
        return Err(Error::MalformedToken);
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

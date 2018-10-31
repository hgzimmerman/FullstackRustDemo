use frank_jwt::{Algorithm, encode, decode};
use rocket::State;
use rocket::http::Status;
use serde_json;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use chrono::Utc;
use log::{warn, info};

use Secret;
use banned_set::BannedSet;

use error::Error;
use error::JwtError;

use wire::user::{Jwt, UserRole, BEARER};


use identifiers::user::UserUuid;

/// Because the JWT struct lives in the wire crate,
/// this NewType is used to define other functions on it.
#[derive(Clone, PartialEq, Debug)]
pub struct ServerJwt(pub Jwt);

impl ServerJwt {
    /// Encodes the JWT, producing a string.
    pub fn encode_jwt_string(&self, secret: &Secret) -> Result<String, JwtError> {
        let header = json!({});
        use rocket_contrib::Value;

        let secret: &String = &secret.0;

        let payload: Value = match serde_json::to_value(&self.0) {
            Ok(x) => x,
            Err(_) => return Err(JwtError::SerializeError),
        };
        match encode(header, secret, &payload, Algorithm::HS256) {
            Ok(x) => return Ok(x),
            Err(_) => return Err(JwtError::EncodeError),
        }
    }

    pub fn decode_jwt_string(jwt_str: &str, secret: &Secret) -> Result<ServerJwt, JwtError> {
        let secret: &String = &secret.0;
        let (_header, payload) = match decode(&jwt_str.to_string(), secret, Algorithm::HS256) {
            Ok(x) => x,
            Err(_) => return Err(JwtError::DecodeError),
        };
        let jwt: Jwt = match serde_json::from_value(payload) {
            Ok(x) => x,
            Err(_) => return Err(JwtError::DeserializeError),
        };
        let jwt = ServerJwt(jwt);
        Ok(jwt)
    }
}


/// Raw JWTs can be gotten via the request
/// This should only be used for reauth.
impl<'a, 'r> FromRequest<'a, 'r> for ServerJwt {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ServerJwt, Error> {
        let jwt: ServerJwt = extract_jwt_from_request(request)?;
        let jwt: ServerJwt = validate_jwt_expiry_time(jwt)?;

        Outcome::Success(jwt)
    }
}


/// Given a request, extract the JWT struct from the headers in the request.
fn extract_jwt_from_request<'a, 'r>(request: &'a Request<'r>) -> request::Outcome<ServerJwt, Error> {
    let keys: Vec<_> = request
        .headers()
        .get("Authorization")
        .collect();
    if keys.len() != 1 {
        return Outcome::Failure((Status::Unauthorized, Error::MissingToken));
    };

    let key = keys[0];

    // You can get the state secret from another request guard
    let secret: &Secret = match request.guard::<State<Secret>>() {
        Outcome::Success(s) => s.inner(),
        _ => {
            warn!("Couldn't get secret from state.");
            return Outcome::Failure((Status::InternalServerError, Error::InternalServerError));
        }
    };

    let authorization_words: Vec<String> = key.to_string()
        .split_whitespace()
        .map(String::from)
        .collect();

    if authorization_words.len() != 2 {
        return Outcome::Failure((Status::Unauthorized, Error::MalformedToken));
    }
    if authorization_words[0] != BEARER {
        return Outcome::Failure((Status::Unauthorized, Error::MalformedToken));
    }
    let jwt_str: &str = &authorization_words[1];

    match ServerJwt::decode_jwt_string(jwt_str, secret) {
        Ok(jwt) => Outcome::Success(jwt),
        Err(_) => {
            info!("Token couldn't be deserialized.");
            Outcome::Failure((Status::Unauthorized, Error::IllegalToken))
        }
    }
}

/// Make sure that the JWT hasn't expired yet.
fn validate_jwt_expiry_time(jwt: ServerJwt) -> request::Outcome<ServerJwt, Error> {
    if jwt.0.exp < Utc::now().naive_utc() {
        info!("Token expired.");
        return Outcome::Failure((Status::Unauthorized, Error::ExpiredToken));
    }
    Outcome::Success(jwt)
}






pub mod user_authorization {
    use super::*;

    trait FromJwt {
        fn from_jwt(jwt: &Jwt) -> Result<Self, RoleError>
        where
            Self: Sized;
        fn get_uuid(&self) -> UserUuid;
    }

    pub enum RoleError {
        InsufficientRights,
    }

    pub struct NormalUser {
        pub user_uuid: UserUuid,
    }
    impl FromJwt for NormalUser {
        fn from_jwt(jwt: &Jwt) -> Result<NormalUser, RoleError> {
            if jwt.user_roles.contains(
                &UserRole::Unprivileged,
            )
            {
                Ok(NormalUser { user_uuid: jwt.sub })
            } else {
                Err(RoleError::InsufficientRights)
            }
        }
        fn get_uuid(&self) -> UserUuid {
            self.user_uuid
        }
    }
    impl<'a, 'r> FromRequest<'a, 'r> for NormalUser {
        type Error = Error;

        fn from_request(request: &'a Request<'r>) -> request::Outcome<NormalUser, Error> {
            extract_role_from_request::<NormalUser>(request)
        }
    }

    pub struct AdminUser {
        pub user_uuid: UserUuid,
    }
    impl FromJwt for AdminUser {
        fn from_jwt(jwt: &Jwt) -> Result<AdminUser, RoleError> {
            if jwt.user_roles.contains(
                &UserRole::Admin,
            )
            {
                Ok(AdminUser { user_uuid: jwt.sub })
            } else {
                Err(RoleError::InsufficientRights)
            }
        }
        fn get_uuid(&self) -> UserUuid {
            self.user_uuid
        }
    }
    impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
        type Error = Error;

        fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser, Error> {
            extract_role_from_request::<AdminUser>(request)
        }
    }

    pub struct ModeratorUser {
        pub user_uuid: UserUuid,
    }
    impl FromJwt for ModeratorUser {
        fn from_jwt(jwt: &Jwt) -> Result<ModeratorUser, RoleError> {
            if jwt.user_roles.contains(
                &UserRole::Moderator,
            )
            {
                Ok(ModeratorUser { user_uuid: jwt.sub })
            } else {
                Err(RoleError::InsufficientRights)
            }
        }

        fn get_uuid(&self) -> UserUuid {
            self.user_uuid
        }
    }
    impl<'a, 'r> FromRequest<'a, 'r> for ModeratorUser {
        type Error = Error;

        fn from_request(request: &'a Request<'r>) -> request::Outcome<ModeratorUser, Error> {
            extract_role_from_request::<ModeratorUser>(request)
        }
    }


    fn extract_role_from_request<'a, 'r, T>(request: &'a Request<'r>) -> request::Outcome<T, Error>
    where
        T: FromJwt,
    {
        // Get the jwt from the request's header
        let jwt: ServerJwt = extract_jwt_from_request(request)?;
        // Make sure that the JWT falls within the time bounds.
        let jwt: ServerJwt = validate_jwt_expiry_time(jwt)?;

        let user = match T::from_jwt(&jwt.0) {
            Ok(user) => user,
            Err(_) => return Outcome::Failure((Status::Forbidden, Error::NotAuthorized { reason: "User does not have that role." })),
        };

        // Check for stateful banned status
        match request.guard::<State<BannedSet>>() {
            Outcome::Success(set) => {
                if set.is_user_banned(&user.get_uuid()) {
                    return Outcome::Failure((Status::Unauthorized, Error::BadRequest));
                }
            }
            _ => {
                warn!("Couldn't get banned set from state.");
                return Outcome::Failure((Status::InternalServerError, Error::InternalServerError));
            }
        }

        Outcome::Success(user)

    }
}

use frank_jwt::{Algorithm, encode, decode};
use rocket::State;
use rocket::http::Status;
use serde_json;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use chrono::{NaiveDateTime, Utc};
use log;

use auth::Secret;
use auth::BannedSet;

use error::WeekendAtJoesError;

use wire::user::{Jwt, UserRole};

pub struct ServerJwt(pub Jwt);

impl ServerJwt {
    pub fn encode_jwt_string(&self, secret: &String) -> Result<String, JwtError> {
        let header = json!({});
        use rocket_contrib::Value;

        let payload: Value = match serde_json::to_value(&self.0) {
            Ok(x) => x,
            Err(_) => return Err(JwtError::SerializeError),
        };
        match encode(header, secret, &payload, Algorithm::HS256) {
            Ok(x) => return Ok(x),
            Err(_) => return Err(JwtError::EncodeError),
        }
    }

    pub fn decode_jwt_string(jwt_str: String, secret: &String) -> Result<Jwt, JwtError> {
        let (_header, payload) = match decode(&jwt_str, secret, Algorithm::HS256) {
            Ok(x) => x,
            Err(_) => return Err(JwtError::DecodeError),
        };
        let jwt: Jwt = match serde_json::from_value(payload) {
            Ok(x) => x,
            Err(_) => return Err(JwtError::DeserializeError),
        };
        Ok(jwt)
    }
}


/// Raw JWTs can be gotten via the request
/// This should only be used for reauth.
impl<'a, 'r> FromRequest<'a, 'r> for ServerJwt {
    type Error = WeekendAtJoesError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ServerJwt, WeekendAtJoesError> {
        let jwt = extract_jwt_from_request(request)?;
        let jwt = validate_jwt_expiry_time(jwt)?;

        Outcome::Success(ServerJwt(jwt))
    }
}



fn extract_jwt_from_request<'a, 'r>(request: &'a Request<'r>) -> request::Outcome<Jwt, WeekendAtJoesError> {
    let keys: Vec<_> = request
        .headers()
        .get("Authorization")
        .collect();
    if keys.len() != 1 {
        return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::MissingToken));
    };

    let key = keys[0];

    // You can get the state secret from another request guard
    let secret: String = match request.guard::<State<Secret>>() {
        Outcome::Success(ref s) => s.0.clone(),
        _ => {
            log::warn!("Couldn't get secret from state.");
            return Outcome::Failure((Status::InternalServerError, WeekendAtJoesError::InternalServerError));
        }
    };

    match ServerJwt::decode_jwt_string(key.to_string(), &secret) {
        Ok(jwt) => Outcome::Success(jwt),
        Err(_) => {
            log::info!("Token couldn't be deserialized.");
            Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::IllegalToken))
        }
    }
}

fn validate_jwt_expiry_time(jwt: Jwt) -> request::Outcome<Jwt, WeekendAtJoesError> {
    if jwt.exp < Utc::now().naive_utc() {
        log::info!("Token expired.");
        return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::ExpiredToken));
    }
    Outcome::Success(jwt)
}





#[derive(Debug, Clone)]
pub enum JwtError {
    DecodeError,
    EncodeError,
    DeserializeError,
    SerializeError,
}



pub mod user_authorization {
    use super::*;
    use log;

    trait FromJwt {
        fn from_jwt(jwt: &Jwt) -> Result<Self, RoleError>
        where
            Self: Sized;
        fn get_id(&self) -> i32;
    }

    pub enum RoleError {
        InsufficientRights,
    }

    pub struct NormalUser {
        //        pub user_name: String,
        pub user_id: i32,
    }
    impl FromJwt for NormalUser {
        fn from_jwt(jwt: &Jwt) -> Result<NormalUser, RoleError> {
            if jwt.user_roles.contains(
                &UserRole::Unprivileged,
            )
            {
                Ok(NormalUser {
                    //                    user_name: jwt.user_name.clone(),
                    user_id: jwt.sub,
                })
            } else {
                Err(RoleError::InsufficientRights)
            }
        }
        fn get_id(&self) -> i32 {
            self.user_id
        }
    }
    impl<'a, 'r> FromRequest<'a, 'r> for NormalUser {
        type Error = WeekendAtJoesError;

        fn from_request(request: &'a Request<'r>) -> request::Outcome<NormalUser, WeekendAtJoesError> {
            extract_role_from_request::<NormalUser>(request)
        }
    }

    pub struct AdminUser {
        //        pub user_name: String,
        pub user_id: i32,
    }
    impl FromJwt for AdminUser {
        fn from_jwt(jwt: &Jwt) -> Result<AdminUser, RoleError> {
            if jwt.user_roles.contains(
                &UserRole::Admin,
            )
            {
                Ok(AdminUser {
                    //                    user_name: jwt.user_name.clone(),
                    user_id: jwt.sub,
                })
            } else {
                Err(RoleError::InsufficientRights)
            }
        }
        fn get_id(&self) -> i32 {
            self.user_id
        }
    }
    impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
        type Error = WeekendAtJoesError;

        fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser, WeekendAtJoesError> {
            extract_role_from_request::<AdminUser>(request)
        }
    }

    pub struct ModeratorUser {
        //        pub user_name: String,
        pub user_id: i32,
    }
    impl FromJwt for ModeratorUser {
        fn from_jwt(jwt: &Jwt) -> Result<ModeratorUser, RoleError> {
            if jwt.user_roles.contains(
                &UserRole::Moderator,
            )
            {
                Ok(ModeratorUser {
                    //                    user_name: jwt.user_name.clone(),
                    user_id: jwt.sub,
                })
            } else {
                Err(RoleError::InsufficientRights)
            }
        }

        fn get_id(&self) -> i32 {
            self.user_id
        }
    }
    impl<'a, 'r> FromRequest<'a, 'r> for ModeratorUser {
        type Error = WeekendAtJoesError;

        fn from_request(request: &'a Request<'r>) -> request::Outcome<ModeratorUser, WeekendAtJoesError> {
            extract_role_from_request::<ModeratorUser>(request)
        }
    }


    fn extract_role_from_request<'a, 'r, T>(request: &'a Request<'r>) -> request::Outcome<T, WeekendAtJoesError>
    where
        T: FromJwt,
    {
        // Get the jwt from the request's header
        let jwt: Jwt = extract_jwt_from_request(request)?;
        // Make sure that the JWT falls within the time bounds.
        let jwt: Jwt = validate_jwt_expiry_time(jwt)?;

        let user = match T::from_jwt(&jwt) {
            Ok(user) => user,
            Err(_) => return Outcome::Failure((Status::Forbidden, WeekendAtJoesError::NotAuthorized { reason: "User does not have that role." })),
        };

        // Check for stateful banned status
        match request.guard::<State<BannedSet>>() {
            Outcome::Success(set) => {
                if set.is_user_banned(&user.get_id()) {
                    return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::BadRequest));
                }
            }
            _ => {
                log::warn!("Couldn't get banned set from state.");
                return Outcome::Failure((Status::InternalServerError, WeekendAtJoesError::InternalServerError));
            }
        }

        Outcome::Success(user)

    }

}

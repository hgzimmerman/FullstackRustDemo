use frank_jwt::{Algorithm, encode, decode};
use rocket::State;
use rocket::http::Status;
use serde_json;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use chrono::{NaiveDateTime, Utc};

use auth::Secret;
use auth::BannedSet;

use error::WeekendAtJoesError;

use requests_and_responses::user::{Jwt, UserRole};

pub struct ServerJwt(pub Jwt);

//#[derive(Clone, Debug, Serialize, Deserialize)]
//pub struct Jwt {
//    pub user_name: String,
//    pub user_id: i32,
//    pub user_roles: Vec<UserRole>,
//    pub token_expire_date: NaiveDateTime,
//}

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

#[derive(Debug, Clone)]
pub enum JwtError {
    DecodeError,
    EncodeError,
    DeserializeError,
    SerializeError,
}



pub mod user_authorization {
    use super::*;

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
        pub user_name: String,
        pub user_id: i32,
    }
    impl FromJwt for NormalUser {
        fn from_jwt(jwt: &Jwt) -> Result<NormalUser, RoleError> {
            if jwt.user_roles.contains(
                &UserRole::Unprivileged,
            )
            {
                Ok(NormalUser {
                    user_name: jwt.user_name.clone(),
                    user_id: jwt.user_id,
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
        pub user_name: String,
        pub user_id: i32,
    }
    impl FromJwt for AdminUser {
        fn from_jwt(jwt: &Jwt) -> Result<AdminUser, RoleError> {
            if jwt.user_roles.contains(
                &UserRole::Admin,
            )
            {
                Ok(AdminUser {
                    user_name: jwt.user_name.clone(),
                    user_id: jwt.user_id,
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
        pub user_name: String,
        pub user_id: i32,
    }
    impl FromJwt for ModeratorUser {
        fn from_jwt(jwt: &Jwt) -> Result<ModeratorUser, RoleError> {
            if jwt.user_roles.contains(
                &UserRole::Moderator,
            )
            {
                Ok(ModeratorUser {
                    user_name: jwt.user_name.clone(),
                    user_id: jwt.user_id,
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
        let keys: Vec<_> = request
            .headers()
            .get("Authorization")
            .collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::MissingToken));
        };



        // You can get the state secret from another request guard
        let secret: String = match request.guard::<State<Secret>>() {
            Outcome::Success(s) => s.0.clone(),
            _ => {
                warn!("Couldn't get secret from state.");
                return Outcome::Failure((Status::InternalServerError, WeekendAtJoesError::InternalServerError));
            }
        };

        let key = keys[0];
        let jwt: Jwt = match ServerJwt::decode_jwt_string(key.to_string(), &secret) {
            Ok(token) => {
                if token.token_expire_date < Utc::now().naive_utc() {
                    info!("Token expired.");
                    return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::ExpiredToken));
                }
                token
            }
            Err(_) => {
                info!("Token couldn't be deserialized.");
                return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::IllegalToken));
            }
        };

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
                warn!("Couldn't get banned set from state.");
                return Outcome::Failure((Status::InternalServerError, WeekendAtJoesError::InternalServerError));
            }
        }

        Outcome::Success(user)

    }

}

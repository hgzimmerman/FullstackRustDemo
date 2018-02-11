use frank_jwt::{Algorithm, encode, decode};
use rocket::State;
use rocket::http::Status;
use serde_json;
use db::user::UserRole;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};
use chrono::{NaiveDateTime, Utc};

use auth::Secret;

use error::WeekendAtJoesError;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub user_name: String,
    pub token_key: String,// The token key may not be needed
    pub user_roles: Vec<UserRole>,
    pub token_expire_date: NaiveDateTime
}

impl Jwt {
    pub fn encode_jwt_string(&self, secret: &String) -> Result<String, JwtError> {
        let header = json!({});
        use rocket_contrib::Value;

        let payload: Value = match serde_json::to_value(self) {
            Ok(x) => x,
            Err(_) => return Err(JwtError::SerializeError)
        };
        match encode(header, secret, &payload, Algorithm::HS256) {
            Ok(x) => return Ok(x),
            Err(_) => return Err(JwtError::EncodeError)
        }
    }

    pub fn decode_jwt_string(jwt_str: String, secret: &String) -> Result<Jwt, JwtError> {
        let (_header, payload) = match decode(&jwt_str, secret, Algorithm::HS256) {
            Ok(x) => x,
            Err(_) => return Err(JwtError::DecodeError)
        };
        let jwt: Jwt = match serde_json::from_value(payload) {
            Ok(x) => x,
            Err(_) => return Err(JwtError::DeserializeError)
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
            where Self: Sized;
    }

    pub enum RoleError {
        InsufficientRights
    }

    pub struct NormalUser{
        pub user_name: String
    }
    impl FromJwt for NormalUser {
        fn from_jwt(jwt: &Jwt) -> Result<NormalUser, RoleError> {
            if jwt.user_roles.contains(&UserRole::Unprivileged){
                Ok(NormalUser{
                    user_name: jwt.user_name.clone()
                })
            }
            else {
                Err(RoleError::InsufficientRights)
            }
        }
    }
    impl<'a, 'r> FromRequest<'a, 'r> for NormalUser {
        type Error = WeekendAtJoesError;

        fn from_request(request: &'a Request<'r>) -> request::Outcome<NormalUser, WeekendAtJoesError> {
            let keys: Vec<_> = request.headers().get("Authorization").collect();
            if keys.len() != 1 {
                return Outcome::Failure((Status::InternalServerError, WeekendAtJoesError::MissingToken));
            };
            // You can get the state secret from another request guard
            let secret: String = match request.guard::<State<Secret>>() {
                Outcome::Success(s) => s.0.clone(),
                _ => return Outcome::Failure((Status::InternalServerError, WeekendAtJoesError::InternalServerError))
            };

            let key = keys[0];
            let jwt: Jwt = match Jwt::decode_jwt_string(key.to_string(), &secret) {
                Ok(token) => {
                    if token.token_expire_date < Utc::now().naive_utc() {
                        return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::ExpiredToken))
                    }
                    token
                }
                Err(e) => return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::IllegalToken)),
            };

            match NormalUser::from_jwt(&jwt) {
                Ok(admin) => Outcome::Success(admin),
                Err(e) => Outcome::Failure((Status::Forbidden, WeekendAtJoesError::NotAuthorized { reason: "User does not have that role."}))
            }
        }
    }

    pub struct AdminUser {
        pub user_name: String
    }
    impl FromJwt for AdminUser {
        fn from_jwt(jwt: &Jwt) -> Result<AdminUser, RoleError> {
            if jwt.user_roles.contains(&UserRole::Admin){
                Ok(AdminUser{
                    user_name: jwt.user_name.clone()
                })
            }
            else {
                Err(RoleError::InsufficientRights)
            }
        }
    }
    impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
        type Error = WeekendAtJoesError;

        fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser, WeekendAtJoesError> {
            let keys: Vec<_> = request.headers().get("Authorization").collect();
            if keys.len() != 1 {
                return Outcome::Failure((Status::InternalServerError, WeekendAtJoesError::MissingToken));
            };
            // You can get the state secret from another request guard
            let secret: String = match request.guard::<State<Secret>>() {
                Outcome::Success(s) => s.0.clone(),
                _ => return Outcome::Failure((Status::InternalServerError, WeekendAtJoesError::InternalServerError))
            };

            let key = keys[0];
            let jwt: Jwt = match Jwt::decode_jwt_string(key.to_string(), &secret) {
                Ok(token) => {
                    if token.token_expire_date < Utc::now().naive_utc() {
                        return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::ExpiredToken))
                    }
                    token
                }
                Err(e) => return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::IllegalToken)),
            };

            match AdminUser::from_jwt(&jwt) {
                Ok(admin) => Outcome::Success(admin),
                Err(e) => Outcome::Failure((Status::Forbidden, WeekendAtJoesError::NotAuthorized { reason: "User does not have that role."}))
            }
        }
    }

    pub struct ModeratorUser {
        pub user_name: String
    }
    impl FromJwt for ModeratorUser {
        fn from_jwt(jwt: &Jwt) -> Result<ModeratorUser, RoleError> {
            if jwt.user_roles.contains(&UserRole::Moderator){
                Ok(ModeratorUser{
                    user_name: jwt.user_name.clone()
                })
            }
            else {
                Err(RoleError::InsufficientRights)
            }
        }
    }
    impl<'a, 'r> FromRequest<'a, 'r> for ModeratorUser {
        type Error = WeekendAtJoesError;

        fn from_request(request: &'a Request<'r>) -> request::Outcome<ModeratorUser, WeekendAtJoesError> {
            let keys: Vec<_> = request.headers().get("Authorization").collect();
            if keys.len() != 1 {
                return Outcome::Failure((Status::InternalServerError, WeekendAtJoesError::MissingToken));
            };
            // You can get the state secret from another request guard
            let secret: String = match request.guard::<State<Secret>>() {
                Outcome::Success(s) => s.0.clone(),
                _ => return Outcome::Failure((Status::InternalServerError, WeekendAtJoesError::InternalServerError))
            };

            let key = keys[0];
            let jwt: Jwt = match Jwt::decode_jwt_string(key.to_string(), &secret) {
                Ok(token) => {
                    if token.token_expire_date < Utc::now().naive_utc() {
                        return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::ExpiredToken))
                    }
                    token
                }
                Err(e) => return Outcome::Failure((Status::Unauthorized, WeekendAtJoesError::IllegalToken)),
            };

            match ModeratorUser::from_jwt(&jwt) {
                Ok(admin) => Outcome::Success(admin),
                Err(e) => Outcome::Failure((Status::Forbidden, WeekendAtJoesError::NotAuthorized { reason: "User does not have that role."}))
            }
        }
    }

}
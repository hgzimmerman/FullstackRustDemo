extern crate frank_jwt;
#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate serde_json;
extern crate chrono;

use chrono::NaiveDateTime;
use serde_json::Value;

use frank_jwt::{Algorithm, encode, decode};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UserRole {
    Unprivileged,
    Moderator,
    Admin,
    Publisher,
}

impl From<UserRole> for i32 {
    fn from(role: UserRole) -> i32 {
        match role {
            UserRole::Unprivileged => 1,
            UserRole::Moderator => 2,
            UserRole::Admin => 3,
            UserRole::Publisher => 4,
        }
    }
}

impl From<i32> for UserRole {
    fn from(number: i32) -> UserRole {
        match number {
            1 => UserRole::Unprivileged,
            2 => UserRole::Moderator,
            3 => UserRole::Admin,
            4 => UserRole::Publisher,
            _ => {
//                warn!("Tried to convert an unsupported number into a user role");
                UserRole::Unprivileged
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub user_name: String,
    pub user_id: i32,
    pub user_roles: Vec<UserRole>,
    pub token_expire_date: NaiveDateTime,
}

impl Jwt {
    pub fn encode_jwt_string(&self, secret: &String) -> Result<String, JwtError> {
        let header = json!({});
//        use rocket_contrib::Value;

        let payload: Value = match serde_json::to_value(self) {
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

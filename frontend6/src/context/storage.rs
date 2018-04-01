
use super::Context;
use failure::Error;

//use chrono::NaiveDateTime;

//// This type is pegged to the server implementation
//// It might make sense to move this into the requests and responses crate.
//#[derive(Clone, Debug)]
//pub struct Jwt {
//    pub user_name: String,
//    pub user_id: i32,
//    pub user_roles: Vec<UserRole>,
//    pub token_expire_date: NaiveDateTime,
//}
//
//impl Jwt {
//    pub fn encode_jwt_string(&self, secret: &String) -> Result<String, JwtError> {
//        let header = json!({});
//        use rocket_contrib::Value;
//
//        let payload: Value = match serde_json::to_value(self) {
//            Ok(x) => x,
//            Err(_) => return Err(JwtError::SerializeError),
//        };
//        match encode(header, secret, &payload, Algorithm::HS256) {
//            Ok(x) => return Ok(x),
//            Err(_) => return Err(JwtError::EncodeError),
//        }
//    }
//
//    pub fn decode_jwt_string(jwt_str: String, secret: &String) -> Result<Jwt, JwtError> {
//        let (_header, payload) = match decode(&jwt_str, secret, Algorithm::HS256) {
//            Ok(x) => x,
//            Err(_) => return Err(JwtError::DecodeError),
//        };
//        let jwt: Jwt = match serde_json::from_value(payload) {
//            Ok(x) => x,
//            Err(_) => return Err(JwtError::DeserializeError),
//        };
//        Ok(jwt)
//    }
//}
//
//// This type is pegged to the server implementation
//// It might make sense to move this into the requests and responses crate.
//#[derive(Debug, Clone, PartialEq)]
//pub enum UserRole {
//    Unprivileged,
//    Moderator,
//    Admin,
//    Publisher,
//}

impl Context {
    pub fn store_jwt(&mut self, jwt: String) {
        self.local_storage.store("JWT", jwt)
    }

    pub fn restore_jwt(&mut self) -> Result<String, Error> {
        self.local_storage.restore("JWT")
    }

    pub fn is_logged_in(&mut self) -> bool {
        return false;
    }

    /// Functionally logs the user out
    pub fn remove_jwt(&mut self) {
        unimplemented!()
    }
}
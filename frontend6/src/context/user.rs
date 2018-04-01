use super::Context;


use chrono::NaiveDateTime;

use base64::decode_config as b64_dec;
use serde_json::Value as JsonValue;
use serde_json;
use failure::Error;
use base64;

//// This type is pegged to the server implementation
//// It might make sense to move this into the requests and responses crate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub user_name: String,
    pub user_id: i32,
    pub user_roles: Vec<UserRole>,
    pub token_expire_date: NaiveDateTime,
}

// This type is pegged to the server implementation
// It might make sense to move this into the requests and responses crate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    Unprivileged,
    Moderator,
    Admin,
    Publisher,
}

    fn extract_payload_from_jwt(jwt_string: String) -> Result<Jwt, String> {
        let payload_segment: &str = jwt_string.split('.').collect::<Vec<&str>>().get(1).ok_or_else(||"JWT should have 3 distinct sections".to_string())?;
        let payload_json: JsonValue = decode_payload(payload_segment).map_err(|_| "JWT payload could not be decoded from base64".to_string())?;
        serde_json::from_value(payload_json).map_err(|_|"JWT payload could not be decoded from JSON".to_string())
    }

    // Taken from frank_jwt source
    fn decode_payload(payload_segment: &str) -> Result<JsonValue, Error> {
    serde_json::from_slice(b64_dec(payload_segment, base64::URL_SAFE)?
        .as_slice())
        .map_err(Error::from)
    }




impl Context {

    fn user_has_role(&mut self, role: &UserRole) -> bool {
        if let Ok(token) = self.restore_jwt() {
            match extract_payload_from_jwt(token) {
                Ok(payload) => payload.user_roles.contains(role),
                Err(e) => {
                    println!("{}", e);
                    false
                }
            }
        }
        else {
            false
        }
    }

    pub fn user_is_unprivileged(&mut self) -> bool {
        self.user_has_role(&UserRole::Unprivileged)
    }
    pub fn user_is_moderator(&mut self) -> bool {
        self.user_has_role(&UserRole::Moderator)
    }
    pub fn user_is_publisher(&mut self) -> bool {
        self.user_has_role(&UserRole::Publisher)
    }
    pub fn user_is_admin(&mut self) -> bool {
        self.user_has_role(&UserRole::Admin)
    }


    pub fn user_id(&mut self) -> Result<i32, ()> {
        if let Ok(token) = self.restore_jwt() {
            match extract_payload_from_jwt(token) {
                Ok(payload) => Ok(payload.user_id),
                Err(e) => {
                    println!("{}", e);
                    Err(())
                }
            }
        }
        else {
            Err(())
        }
    }

    pub fn user_name(&mut self) -> Result<String, ()> {
        if let Ok(token) = self.restore_jwt() {
            match extract_payload_from_jwt(token) {
                Ok(payload) => Ok(payload.user_name),
                Err(e) => {
                    println!("{}", e);
                    Err(())
                }
            }
        }
        else {
            Err(())
        }
    }
}
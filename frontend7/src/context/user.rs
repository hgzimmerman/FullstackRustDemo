use super::Context;


//use chrono::{NaiveDateTime, Utc};

use base64::decode_config as b64_dec;
use serde_json::Value as JsonValue;
use serde_json;
use failure::Error;
use base64;

use wire::user::{UserRole, Jwt};


#[derive(Fail, Debug)]
enum JwtError {
    #[fail(display = "JWT should have 3 distinct sections")]
    UnexpectedNumberOfSections,
    #[fail(display = "JWT JSON payload could not be converted from Base64")]
    Base64DecodeFailure,
    #[fail(display = "Value representing JWT could not be converted from json")]
    JsonDecodeFailure,
}

pub fn extract_payload_from_jwt(jwt_string: String) -> Result<Jwt, Error> {
    let payload_segment: &str = jwt_string
        .split('.')
        .collect::<Vec<&str>>()
        .get(1)
        .ok_or_else(|| Error::from(JwtError::UnexpectedNumberOfSections))?;
    let payload_json: JsonValue = decode_payload(payload_segment)
        .map_err(|_| Error::from(JwtError::Base64DecodeFailure))?;
    serde_json::from_value(payload_json)
        .map_err(|_| Error::from(JwtError::JsonDecodeFailure))
}

// Taken from frank_jwt source
fn decode_payload(payload_segment: &str) -> Result<JsonValue, Error> {
    serde_json::from_slice(
        b64_dec(payload_segment, base64::URL_SAFE)?
            .as_slice(),
    ).map_err(Error::from)
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
        } else {
            false
        }
    }

    pub fn user_is_unprivileged(&mut self) -> bool {
        self.user_has_role(
            &UserRole::Unprivileged,
        )
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


    pub fn user_id(&mut self) -> Result<i32, Error> {
        let token = self.restore_jwt()?;
        let payload = extract_payload_from_jwt(token)?;
        Ok(payload.sub)
    }

/*    pub fn user_name(&mut self) -> Result<String, Error> {
        let token = self.restore_jwt()?;
        let payload = extract_payload_from_jwt(token)?;
        Ok(payload.user_name)
    }*/

    fn user_auth_expire_date(&mut self) -> Result<i64, Error> {
        let token = self.restore_jwt()?;
        let payload = extract_payload_from_jwt(token)?;
        Ok(payload.exp)
    }

    pub fn has_token_expired(&mut self) -> bool {
        match self.user_auth_expire_date() {
            Ok(_expire_date) => {
                false // TODO impl me
            } Err(_e) => {
                true
            }
        }
//                let now = Utc::now().naive_utc();
//                if expire_date < now { true } else { false }

        // TODO, handle dates intelligently in frontend.

    }
}

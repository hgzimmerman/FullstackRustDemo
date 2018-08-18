
use yew::services::storage::{StorageService, Area};
use yew::prelude::worker::*;

use base64::decode_config as b64_dec;
use serde_json::Value as JsonValue;
use serde_json;
use failure::Error;
use base64;

use wire::user::{UserRole, Jwt};

use chrono::NaiveDateTime;
use identifiers::user::UserUuid;
use std::collections::HashSet;


#[derive(Fail, Debug)]
enum JwtError {
    #[fail(display = "JWT should have 3 distinct sections")]
    UnexpectedNumberOfSections,
    #[fail(display = "JWT JSON payload could not be converted from Base64")]
    Base64DecodeFailure,
    #[fail(display = "Value representing JWT could not be converted from json")]
    JsonDecodeFailure,
}

pub fn extract_payload_from_jwt(jwt_string: &str) -> Result<Jwt, Error> {
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

/// Decodes the payload section of the token using a base 64 decoder.
///
/// Taken from frank_jwt source
fn decode_payload(payload_segment: &str) -> Result<JsonValue, Error> {
    serde_json::from_slice(
        b64_dec(payload_segment, base64::URL_SAFE)?
            .as_slice(),
    ).map_err(Error::from)
}




fn user_has_role(storage: &mut StorageService, role: UserRole) -> bool {
    if let Ok(token) = restore_jwt(storage) {
        match extract_payload_from_jwt(&token) {
            Ok(payload) => payload.user_roles.contains(&role),
            Err(e) => {
                println!("{}", e);
                false
            }
        }
    } else {
        false
    }
}

pub fn user_is_unprivileged(storage: &mut StorageService) -> bool {
    user_has_role(
        storage,
        UserRole::Unprivileged,
    )
}
pub fn user_is_moderator(storage: &mut StorageService) -> bool {
    user_has_role(storage, UserRole::Moderator)
}
pub fn user_is_publisher(storage: &mut StorageService) -> bool {
    user_has_role(storage, UserRole::Publisher)
}
pub fn user_is_admin(storage: &mut StorageService) -> bool {
    user_has_role(storage, UserRole::Admin)
}


/// Gets the user uuid from the token.
pub fn user_id(storage: &mut StorageService) -> Result<UserUuid, Error> {
    let token = restore_jwt(storage)?;
    let payload = extract_payload_from_jwt(&token)?;
    Ok(payload.sub)
}


/// Gets the expiry date from the token.
pub fn user_auth_expire_date(storage_service: &mut StorageService) -> Result<NaiveDateTime, Error> {
    let token = restore_jwt(storage_service)?;
    let payload = extract_payload_from_jwt(&token)?;
    Ok(payload.exp)
}

/// Gets current datetime.
pub fn get_now() -> NaiveDateTime {
    use stdweb::Value;
    use stdweb::unstable::TryInto;
    // Get current unix timestamp from js
    let current_time_as_seconds: Value = js! {
        var d = new Date();
        return Math.floor(d.getTime() / 1000);
    };

    let current_time_as_seconds: i64 = current_time_as_seconds.try_into().expect("Couldn't convert local timestamp int into Rust i64");
    const NANOS: u32 = 0;
    NaiveDateTime::from_timestamp(current_time_as_seconds, NANOS)
}


/// The token is valid if it exists and if it's expiry date is before now
pub fn get_token_if_valid(storage_service: &mut StorageService) -> Option<String> {
    if let Ok(token) = restore_jwt(storage_service) {
        if let Ok(payload) = extract_payload_from_jwt(&token){
            let expiry_date = payload.exp;
            let now = get_now();
            if now < expiry_date {
                return Some(token)
            }
        }
    }
    None
}



pub fn store_jwt(storage_service: &mut StorageService, jwt: String) {
    let jwt: Result<String, Error> = Ok(jwt);
    storage_service.store("JWT", jwt)
}

pub fn restore_jwt(storage_service: &mut StorageService) -> Result<String, Error> {
    storage_service.restore("JWT")
}

pub fn is_logged_in(storage_service: &mut StorageService) -> bool {
    restore_jwt(storage_service).is_ok()
}

/// Functionally logs the user out
pub fn remove_jwt(storage_service: &mut StorageService) {
    storage_service.remove("JWT");
}



#[derive(Serialize, Deserialize)]
pub enum LoginRequest {
    Logout,
    Login {
        jwt_string: String
    },
    Query
}

#[derive(Serialize, Deserialize)]
pub enum LoginResponse {
    LoggedOut,
    LoggedIn(Jwt)
}

pub struct LoginAgent {
    storage_service: StorageService,
    subscribers: HashSet<HandlerId>,
    link: AgentLink<LoginAgent>
}

impl Transferable for LoginResponse {}
impl Transferable for LoginRequest {}


impl Agent for LoginAgent
{
    type Reach = Context;
    type Message = ();
    type Input = LoginRequest ;
    type Output = LoginResponse;

    fn create(link: AgentLink<Self>) -> Self {
        LoginAgent {
            storage_service: StorageService::new(Area::Local),
            subscribers: HashSet::new(),
            link
        }
    }

    fn update(&mut self, _msg: Self::Message) {

    }

    fn connected(&mut self, who: HandlerId) {
        self.subscribers.insert(who);
    }

    fn handle(&mut self, request: Self::Input, who: HandlerId) {
        match request {
            LoginRequest::Login{jwt_string} => {
                if let Ok(jwt) = extract_payload_from_jwt(&jwt_string) {
                    // Only store the jwt if it is valid
                    store_jwt(&mut self.storage_service, jwt_string);
                    for sub in self.subscribers.iter().filter(|s| *s != &who) {
                        self.link.response(*sub, LoginResponse::LoggedIn(jwt.clone()));
                    }
                }
                else {
                    error!("Could not convert jwt string to usable JWT type: {}", jwt_string)
                }
            }
            LoginRequest::Logout => {
                remove_jwt(&mut self.storage_service);
                for sub in self.subscribers.iter().filter(|s| *s != &who) {
                    self.link.response(*sub, LoginResponse::LoggedOut);
                }
            }
            LoginRequest::Query => {

                let response = if let Ok(jwt_string) = restore_jwt(&mut self.storage_service) {
                    if let Ok(jwt) = extract_payload_from_jwt(&jwt_string) {
                        LoginResponse::LoggedIn(jwt)
                    } else {
                        error!("Could not convert jwt string to usable JWT type: {}", jwt_string);
                        LoginResponse::LoggedIn(Jwt::default())
                    }
                } else {
                    LoginResponse::LoggedOut
                };
                self.link.response(who, response);
            }
        }
    }

    fn disconnected(&mut self, who: HandlerId) {
        self.subscribers.remove(&who);
    }
}

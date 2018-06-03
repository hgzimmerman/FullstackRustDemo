#[cfg(feature="rocket_support")]
extern crate rocket;

extern crate uuid;

#[macro_use]
extern crate serde_derive;

pub mod user;
pub mod article;
pub mod forum;
pub mod post;
pub mod thread;
pub mod bucket;
pub mod question;
pub mod answer;
pub mod chat;
pub mod message;



#[cfg(feature="rocket_support")]
use uuid::Uuid;
#[cfg(feature="rocket_support")]
use std::borrow::Cow;
#[cfg(feature="rocket_support")]
use rocket::http::RawStr;

#[cfg(feature="rocket_support")]
#[inline]
fn uuid_from_param<'a>(param: &'a RawStr) -> Result<Uuid, &'a RawStr> {
    let s: Cow<str> = param.percent_decode().map_err(|_| param)?;
    Uuid::parse_str(&s)
        .map_err(|_| RawStr::from_str("Couldn't parse Uuid"))
}


#[cfg(feature="rocket_support")]
use rocket::request::FormItems;

/// The param name should correspond to the key used in getting the UUID.
/// So in a route structure like: `../users?user_uuid=12345...` The `user_uuid` section should be the param_name.
#[cfg(feature="rocket_support")]
#[inline]
fn uuid_from_form<'f>(items: &mut FormItems<'f>, strict: bool, param_name: &'static str) -> Result<Uuid, ()> {

    // In practice, we'd use a more descriptive error type.
    let mut uuid = None;

    for (key, value) in items {

        if key.as_str() == param_name {
            let uuid_str = value.url_decode().map_err(|_| ())?;
            let parsed_uuid: Uuid = Uuid::parse_str(&uuid_str).map_err(|_| ())?;
            uuid = Some(parsed_uuid)
        } else if strict {
            return Err(())
        }
        // allow extra value when not strict
    }

    uuid.ok_or(())
}

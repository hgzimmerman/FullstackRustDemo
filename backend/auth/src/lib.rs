//! The auth module deals with authenticating users on the site.
//! Passwords are hashed with scrypt.
//! JSON Web Tokens are returned to the user.
//! JWTs should be included in http requests to the site under the `Authorization` header.
//! Because of signature checking, the server can trust the contents of the JWT payload and can use them to guard access to protected APIs.
//! FromRequest is implemented for some dummy user types.
//! They will only succeed in creating themselves if the JWT contains the role the user type corresponds to.
//! By specifying one of these user types on a routable method, rocket will not route the request to it unless it can resolve the role in the JWT in the request header.

#[cfg(feature = "rocket_support")]
extern crate rocket;
#[cfg(feature = "rocket_support")]
extern crate rocket_contrib;

extern crate chrono;
extern crate crypto;
extern crate frank_jwt;
extern crate wire;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate error;
extern crate identifiers;
extern crate rand;
extern crate simplelog;

mod banned_set;
mod jwt;
mod password;
mod secret;

pub use banned_set::BannedSet;
pub use jwt::{
    user_authorization,
    ServerJwt,
};
pub use password::{
    hash_password,
    verify_hash,
};
pub use secret::Secret;

#[cfg(test)]
mod test;

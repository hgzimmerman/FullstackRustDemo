//! The routes module contains the routes that are used by Rocket.
//! The methods in this module are responsible for deserializing and validating the incoming requests and sending them on to their respective `db` methods.

pub mod static_file;
pub mod article;
pub mod user;
pub mod auth;
pub mod bucket_questions;
pub mod forum;


pub use self::article::*;
pub use self::static_file::*;
pub use self::user::*;
pub use self::auth::*;
// these are mostly unused
pub use self::bucket_questions::*;
pub use self::forum::*;
use rocket::Route;



/// Convienence trait that specifies that implementors must package up their routes into a vector 
/// and a path so that consumption by Rocket's mount() function is painless.
/// Every route that should be mounted in rocket must be specified in ROUTES.
/// The PATH is used to format the path, inserting an `/api` before it to distinguish it from other possible paths.
pub trait Routable {
    const ROUTES: &'static Fn() -> Vec<Route>;
    const PATH: &'static str;
}


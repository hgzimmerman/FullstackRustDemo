//! The routes module contains the routes that are used by Rocket.
//! The methods in this module are responsible for deserializing and validating the incoming requests and sending them on to their respective `db` methods.

pub mod answer;
pub mod article;
pub mod auth;
pub mod bucket;
pub mod chat;
pub mod forum;
pub mod message;
pub mod post;
pub mod question;
pub mod static_file;
pub mod thread;
pub mod user;

pub use self::{
    article::*,
    auth::*,
    static_file::*,
    user::*,
};
// these are mostly unused
pub use self::{
    bucket::*,
    forum::*,
};

use rocket::Route;

/// Abstracts away a common closure that is used to convert the wrapped values of a vector
/// into another type.
pub fn convert_vector<T, W>(vec: Vec<T>) -> Vec<W>
where
    W: From<T>,
{
    vec.into_iter().map(W::from).collect()
}

/// Convienence trait that specifies that implementors must package up their routes into a vector
/// and a path so that consumption by Rocket's mount() function is painless.
/// Every route that should be mounted in rocket must be specified in ROUTES.
/// The PATH is used to format the path, inserting an `/api` before it to distinguish it from other possible paths.
pub trait Routable {
    const ROUTES: &'static Fn() -> Vec<Route>;
    const PATH: &'static str;
}

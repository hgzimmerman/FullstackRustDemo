//! Contains all types that will be send between the frontend and the backend.
//! This allows both the frontend and the backend to use the same types.

extern crate identifiers;

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate uuid;

#[cfg(feature = "rocket_support")]
extern crate rocket;

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
pub mod login;




/// Abstracts away a common closure that is used to convert the wrapped values of a vector
/// into another type.
///
/// Because it is common for wire types to be converted to project-specific types,
/// and this is often done over lists of those types, this function is proveded here.
pub fn convert_vector<T, W>(vec: Vec<T>) -> Vec<W>
where
    W: From<T>,
{
    vec.into_iter().map(W::from).collect()
}

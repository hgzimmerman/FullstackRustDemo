//! Contains all types that will be send between the frontend and the backend.
//! This allows both the frontend and the backend to use the same types.
#[macro_use]
extern crate serde_derive;
extern crate chrono;

pub mod user;
pub mod article;
pub mod forum;
pub mod post;
pub mod thread;
pub mod bucket;
pub mod question;
pub mod answer;
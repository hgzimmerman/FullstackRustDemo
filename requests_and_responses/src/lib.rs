//! Contains all types that will be send between the frontend and the backend.
//! This allows both the frontend and the backend to use the same types.
#[macro_use]
extern crate serde_derive;

pub mod user;
pub mod article;
pub mod forum;
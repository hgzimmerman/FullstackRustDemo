//! This is used as a set of common behaviors used for integration testing.

extern crate auth as auth_lib;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
extern crate migrations_internals;

extern crate server;
extern crate rocket;

mod query_helper;
mod database_error;
pub mod setup;
pub mod constants;
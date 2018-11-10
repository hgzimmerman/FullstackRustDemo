//! This crate is where fixtures used by the DB and server test suites should be defined.

extern crate auth;
extern crate db;
extern crate diesel;
#[macro_use]
extern crate lazy_static;

extern crate chrono;

pub mod fixtures;

use diesel::PgConnection;

/// The Fixture trait should be implemented for collections of data used in testing.
/// Because it can be instantiated using just a connection to the database,
/// it allows the creation of the type in question and allows data generated at row insertion time
/// (UUIDs) to be made available to the body of tests.
pub trait Fixture {
    fn generate(conn: &PgConnection) -> Self;
}

/// Because some tests may not require any initial database state, but still utilize the connection,
/// This Fixture is provided to meet that need.
pub struct EmptyFixture;

impl Fixture for EmptyFixture {
    fn generate(_conn: &PgConnection) -> Self {
        EmptyFixture
    }
}

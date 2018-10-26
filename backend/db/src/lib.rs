//! This module is responsible for facilitating interaction with the database.
//! Pools and Connections are defined which allow a pool to be specified at startup, and for routes to request a connection from the pool.
//! The files in this module contain functions that interact with the type specified by the filename.
//!
//! These functions are analogous to stored procedures with the caveat that performance may be lower
//! due to aggregation and organization of types belonging to different tables occurs server-side
//! instead of on the database.


// #![feature(use_extern_macros)]
#![feature(test)]
// TODO remove this once the use of macros stops emitting a stupid quantity of warnings
#![allow(proc_macro_derive_resolution_fallback)]

#![feature(drain_filter)]
extern crate test;

#[macro_use]
extern crate db_proc_macros;
extern crate error;
extern crate wire;
extern crate auth as auth_lib;

#[macro_use]
extern crate diesel;
extern crate uuid;


extern crate slug;
extern crate rand;
extern crate chrono;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate pool;

extern crate rocket;
extern crate identifiers;

#[macro_use(log)]
extern crate log;
extern crate simplelog;




use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
//use r2d2;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use error::ErrorFormatter;
use error::JoeResult;
use diesel::Identifiable;
// use diesel::Insertable;
// use diesel::Queryable;

mod diesel_extensions;

mod calls;
pub use calls::*;

pub mod schema;

mod conversions;
//mod auth;


pub use user::User;
pub use article::Article;
pub use forum::{Forum, NewForum};
pub use thread::{Thread, NewThread};
pub use post::Post;
pub use bucket::Bucket;
pub use question::Question;
pub use answer::Answer;
pub use chat::Chat;
pub use message::Message;

use pool::Pool;


///// Holds a bunch of connections to the database and hands them out to routes as needed.
//pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
//
pub const DATABASE_URL: &'static str = env!("DATABASE_URL");
//
///// Initializes the pool.
//pub fn init_pool(db_url: &str) -> Pool {
//    //    let config = r2d2::Config::default();
//    let manager = ConnectionManager::<PgConnection>::new(db_url);
//    r2d2::Pool::new(manager).expect(
//        "db pool",
//    )
//}

/// Wrapper for PgConnection.
/// This type can be used in route methods to grab a DB connection from the managed pool.
pub struct Conn(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Conn {
    //    #[cfg(test)]
    pub fn new(pooled_connection: r2d2::PooledConnection<ConnectionManager<PgConnection>>) -> Conn {
        Conn(pooled_connection)
    }
}


impl Deref for Conn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//impl Deref for Pool {
//    type Target = Option<PgConnection>;
//    fn deref(&self) -> &Self::Target {
//        &match self.get()  {
//            Ok(conn) => Some(conn.0),
//            Err(_) => None
//        }
//    }
//}

//
//trait GetPgConnection {
//    /// None indicates that all connections are occupied and that an error should be returned.
//    /// Some represents the connection
//    fn get_conn(&self) -> Result<&PgConnection, ()>;
//}
//
//impl GetPgConnection for Mutex<Pool> {
//    fn get_conn(&self) -> Result<&PgConnection, ()> {
//        match self.get() {
//            Ok(conn) => {
//                Ok(conn.deref())
//            },
//            Err(_) => Err(()) // TODO this should be a timeout error, because the pool.get() internally waits for a timeout.// TODO this should represent a SERVICE_UNAVAILABLE or possibly wait for a conn to free until a timeout occurs
//        }
//    }
//}

//
//impl GetConnection for Conn {
//    fn get_conn(&self) -> Result<Conn, ()> {
//        Ok(self)
//    }
//}


impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    // Gets the pool from the request and extracts a reference to a connection which is then wrapped in a Conn() and handed to route.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}





use uuid::Uuid;

pub trait CreatableUuid<T> {
    fn create(insert: T, conn: &PgConnection) -> JoeResult<Self>
    where
        Self: Sized;
}

pub trait RetrievableUuid<'a> {
    fn get_by_uuid(id: Uuid, conn: &PgConnection) -> JoeResult<Self>
    where
        Self: 'a + Sized,
        &'a Self: Identifiable;

    fn get_all(conn: &PgConnection) -> JoeResult<Vec<Self>>
    where
        Self: 'a + Sized,
        &'a Self: Identifiable;

    fn exists(id: Uuid, conn: &PgConnection) -> JoeResult<bool>
    where
        Self: 'a + Sized,
        &'a Self: Identifiable;

    // fn get_paginated(page_index: i64, page_size: i64, conn: &Conn) -> Result<Vec<Self>, WeekendAtJoesError>
    //     where
    //         Self: Sized;
}

trait DeletableUuid<'a> {
    /// The delete operation will fail if any children exist: `ForeignKeyViolation`.
    /// A separate, safe-delete operation should be implemented that cleans up all children before this runs.
    fn delete_by_id(id: Uuid, conn: &PgConnection) -> JoeResult<Self>
    where
        Self: ErrorFormatter,
        Self: 'a + Sized,
        &'a Self: Identifiable;
}


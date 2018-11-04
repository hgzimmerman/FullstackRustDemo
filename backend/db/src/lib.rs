//! This module is responsible for facilitating interaction with the database.
//! Pools and Connections are defined which allow a pool to be specified at startup, and for routes to request a connection from the pool.
//! The files in this module contain functions that interact with the type specified by the filename.
//!
//! These functions are analogous to stored procedures with the caveat that performance may be lower
//! due to aggregation and organization of types belonging to different tables occurs server-side
//! instead of on the database.

//#![recursion_limit="512"]
//#![recursion_limit="256"]
// #![feature(use_extern_macros)]
#![feature(test)]
// TODO remove this once the use of macros stops emitting a stupid quantity of warnings
#![allow(proc_macro_derive_resolution_fallback)]

#![feature(drain_filter)]
extern crate test;

//#[macro_use]
//extern crate db_proc_macros;
extern crate error;
extern crate wire;
extern crate auth as auth_lib;

#[macro_use]
extern crate diesel;
extern crate uuid;

#[cfg(feature = "rocket_support")]
extern crate rocket;


extern crate slug;
extern crate rand;
extern crate chrono;
extern crate r2d2_diesel;
extern crate r2d2;
extern crate pool;

extern crate identifiers;

extern crate typename;
#[macro_use]
extern crate typename_derive;

#[macro_use(log)]
extern crate log;
extern crate simplelog;



// use diesel::Insertable;
// use diesel::Queryable;

mod diesel_extensions;

mod calls;
pub use crate::calls::*;

pub mod schema;

mod conversions;


pub use crate::user::User;
pub use crate::article::Article;
pub use crate::forum::{Forum, NewForum};
pub use crate::thread::{Thread, NewThread};
pub use crate::post::Post;
pub use crate::bucket::Bucket;
pub use crate::question::Question;
pub use crate::answer::Answer;
pub use crate::chat::Chat;
pub use crate::message::Message;


#[cfg(feature = "rocket_support")]
pub use self::rocket_support::*;
#[cfg(feature = "rocket_support")]
pub mod rocket_support {
    use super::*;
    use pool::Pool;

    use diesel::pg::PgConnection;
    use r2d2_diesel::ConnectionManager;
    use std::ops::Deref;
    use rocket::http::Status;
    use rocket::request::{self, FromRequest};
    use rocket::{Request, State, Outcome};
//    use diesel::Identifiable;



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
}

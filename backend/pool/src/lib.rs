//! This crate contains basic wrappers around r2d2 and facilitates easy access to pooled connections.
//!

extern crate diesel;
extern crate r2d2;
#[cfg(feature = "rocket_support")]
extern crate rocket;

use diesel::{
    pg::PgConnection,
    r2d2::ConnectionManager,
    Connection,
};
use r2d2::{
    Pool as R2D2Pool,
    PooledConnection,
};

pub const DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Holds a bunch of connections to the database and hands them out to routes as needed.
pub type Pool = R2D2Pool<ConnectionManager<PgConnection>>;
pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;

/// Initializes the pool.
pub fn init_pool(db_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool")
}

pub fn create_single_connection(db_url: &str) -> PgConnection {
    PgConnection::establish(db_url).expect("Database not available. maybe provided url is wrong, or database is down?")
}

#[cfg(feature = "rocket_support")]
pub use self::rocket_support::*;
#[cfg(feature = "rocket_support")]
pub mod rocket_support {
    use super::*;

    use rocket::{
        http::Status,
        request::{
            self,
            FromRequest,
        },
        Outcome,
        Request,
        State,
    };
    use std::ops::Deref;

    /// Wrapper for PgConnection.
    /// This type can be used in route methods to grab a DB connection from the managed pool.
    pub struct Conn(PooledConn);

    impl Conn {
        //    #[cfg(test)]
        pub fn new(pooled_connection: PooledConn) -> Conn {
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

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2;
// use r2d2::{PooledConnection,GetTimeout, Config};

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
//use dotenv::dotenv;


// pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
// //    dotenv().ok();

//    let database_url = env::var("DATABASE_URL")
//        .expect("DATABASE_URL must be set");
//    let config = Config::default();
//    let manager = ConnectionManager::<PgConnection>::new(database_url);
//    Pool::new(config, manager).expect("Failed to create pool.")
// }

// lazy_static! {
//    pub static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = create_db_pool();
// }

// pub struct DB(PooledConnection<ConnectionManager<PgConnection>>);

// impl D {
//    pub fn conn(&self) -> &PgConnection {
//        &*self.0
//    }
// }

// impl<'a, 'r> FromRequest<'a, 'r> for DB {
//    type Error = GetTimeout;
//    fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
//        match DB_POOL.get() {
//            Ok(conn) => Outcome::Success(DB(conn)),
//            Err(e) => Outcome::Failure((Status::InternalServerError, e)),
//        }
//    }
// }




pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub const DATABASE_FILE: &'static str = env!("DATABASE_URL");

pub fn init_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_FILE);
    r2d2::Pool::new(config, manager).expect("db pool")
}

pub struct Conn(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Conn {
    pub (crate) fn new( pooled_connection: r2d2::PooledConnection<ConnectionManager<PgConnection>> ) -> Conn {
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

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
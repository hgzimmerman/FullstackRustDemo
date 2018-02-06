

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2;
use r2d2::{PooledConnection,GetTimeout,Pool, Config};

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
//use dotenv::dotenv;
use std::env;
use std::sync::Mutex;


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

pub type DbConn = Mutex<PgConnection>;
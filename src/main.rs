#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(rand)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_simpleauth as auth;
extern crate uuid;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;
extern crate simplelog;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2_diesel;
extern crate r2d2;

extern crate bcrypt;

use rocket::Rocket;
use rocket_contrib::Json;
use uuid::Uuid;

mod routes;
use routes::*;


use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};


use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::fs::File;


fn main() {

    const LOGFILE_NAME: &'static str = "weekend.log";
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Trace, Config::default(), File::create(LOGFILE_NAME).unwrap()),
        ]
    ).unwrap();

    init_rocket().launch();
}

///Initialize the webserver
pub fn init_rocket() -> Rocket {
    rocket::ignite()
//        .manage(init_pool())
        .mount("/", routes![static_file::files])
        .mount( &format_api(User::PATH), User::ROUTES() )
        .mount( &format_api(Article::PATH), Article::ROUTES() )
        .mount( &format_api(Login::PATH), Login::ROUTES() )
}



// An alias to the type for a pool of Diesel SQLite connections.
type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Initializes a database pool.
fn init_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
    r2d2::Pool::new(config, manager).expect("db pool")
}



// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

///Path should be an &str that starts with a /
fn format_api(path: &str) -> String {
    String::from("/api") + path
}
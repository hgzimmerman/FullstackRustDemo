#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(rand)]
#![recursion_limit="128"]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
// extern crate rocket_simpleauth as auth;
extern crate uuid;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;
extern crate simplelog;

extern crate frank_jwt;

#[macro_use] extern crate diesel;
//#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel_infer_schema;
// #[macro_use] extern crate diesel_derive_enum;
extern crate chrono;
extern crate r2d2_diesel;
extern crate r2d2;
// #[macro_use]
// extern crate lazy_static;

// extern crate bcrypt;
extern crate crypto;

extern crate rand;

use rocket::Rocket;
use std::sync::Mutex;
use std::collections::HashMap;

mod routes;
use routes::*;
mod db;
mod auth;
mod error;
use auth::Secret;
use db::user::User;
use db::article::Article;
use db::forum::Forum;
use db::forum::Thread;

extern crate requests_and_responses;


use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::fs::File;

pub use db::schema; // schema internals can be accessed via db::schema::, or via schema::


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


    let mut bucket_sessions: BucketSessions = BucketSessions(HashMap::new());
    bucket_sessions.0.insert("bucket".to_string(), Bucket::new());
    // let database_url = env::var("DATABASE_URL")
    //     .expect("DATABASE_URL must be set");

    let mutexed_bucket_sessions = Mutex::new(bucket_sessions);

    let secret = Secret::generate();

    rocket::ignite()
        .manage(db::init_pool())
        .manage(secret)
        .manage(mutexed_bucket_sessions)
        .mount("/", routes![static_file::files, static_file::js, static_file::app, static_file::wasm])
        .mount( &format_api(User::PATH), User::ROUTES() )
        .mount( &format_api(Article::PATH), Article::ROUTES() )
        .mount( &format_api(Auth::PATH), Auth::ROUTES() )
        .mount( &format_api(Bucket::PATH), Bucket::ROUTES() )
        .mount( &format_api(Forum::PATH), Forum::ROUTES())
        .mount( &format_api(Thread::PATH), Thread::ROUTES())

}


///Path should be an &str that starts with a /
fn format_api(path: &str) -> String {
    String::from("/api") + path
}


use std::sync::{Once, ONCE_INIT};

static INIT: Once = ONCE_INIT;

/// Setup function that is only run once, even if called multiple times.
pub fn test_setup() {
    INIT.call_once(|| {

        const LOGFILE_NAME: &'static str = "weekend_test.log";
        CombinedLogger::init(
            vec![
                TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
                WriteLogger::new(LogLevelFilter::Trace, Config::default(), File::create(LOGFILE_NAME).unwrap()),
            ]
        ).unwrap();
    });
}
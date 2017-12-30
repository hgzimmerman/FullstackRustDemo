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
mod db;


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
        .mount("/", routes![static_file::files, static_file::wasm])
        .mount( &format_api(User::PATH), User::ROUTES() )
        .mount( &format_api(Article::PATH), Article::ROUTES() )
        .mount( &format_api(Login::PATH), Login::ROUTES() )
}




///Path should be an &str that starts with a /
fn format_api(path: &str) -> String {
    String::from("/api") + path
}
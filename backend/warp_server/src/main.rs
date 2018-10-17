#[macro_use]
extern crate warp;
extern crate db;
extern crate wire;
extern crate auth;
extern crate uuid;
extern crate identifiers;
extern crate error as joe_error;

extern crate serde;
extern crate simplelog;

extern crate r2d2;
extern crate r2d2_diesel;
extern crate diesel;
#[macro_use]
extern crate lazy_static;

#[macro_use] extern crate log;
extern crate pretty_env_logger;

mod db_integration;
mod routes;
mod jwt;
mod error;
mod uuid_integration;
mod logging;
mod util;

//use warp::Filter;
//use wire::user::UserResponse;
//use uuid::Uuid;

//use serde::Deserialize;

//use warp::reject::Rejection;
//use warp::reply::Reply;
//use warp::filters::BoxedFilter;
//use std::env;

use self::logging::setup_logging;

fn main() {

    setup_logging();

    let routes = self::routes::api();

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
}

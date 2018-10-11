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

use warp::Filter;
//use wire::user::UserResponse;
//use uuid::Uuid;

use core::fmt::Debug;
//use serde::Deserialize;

use warp::reject::Rejection;
//use warp::reply::Reply;
//use warp::filters::BoxedFilter;
//use std::env;


fn main() {
//    if env::var_os("RUST_LOG").is_none() {
//         Set `RUST_LOG=todos=debug` to see debug logs,
//         this only shows access logs.
//        env::set_var("RUST_LOG", "auth=info");
//    }

//    pretty_env_logger::init();
    setup_logging();

    let routes = self::routes::api();



    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
}



pub fn index_filter() -> impl Filter<Extract = (&'static str,), Error = Rejection> {
    warp::index().map(|| "Index page")
}


/// Sets up logging for the server
pub fn setup_logging() {
    const LOGFILE_NAME: &'static str = "weekend.log";
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, simplelog::Config::default())
            .expect("Couldn't get terminal logger"),
        WriteLogger::new(
            LevelFilter::Debug,
            simplelog::Config::default(),
            File::create(LOGFILE_NAME).expect(
                "Couldn't create logfile",
            )
        ),
    ]).expect("Can't get logger.");
}


// TODO move these to a util file
use warp::Reply;
use serde::Serialize;
use wire::convert_vector;
use simplelog::CombinedLogger;
use simplelog::TermLogger;
use simplelog::WriteLogger;
use std::fs::File;
use simplelog::LevelFilter;

/// Util function that makes replying easier
pub fn convert_and_json<T, U>(source: T) -> impl Reply where
    T: Into<U>,
    U: Serialize
{
    let target: U = source.into();
    warp::reply::json(&target)
}


/// Converts a vector of T to a vector of U then converts the U vector to a JSON reply.
pub fn convert_vector_and_json<T, U>(source: Vec<T>) -> impl Reply where
    U: From<T>,
    U: Serialize
{
    let target: Vec<U> = convert_vector(source);
    warp::reply::json(&target)
}


pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete
}

pub fn log_attach(method: HttpMethod, text: &str) {
    let method: &str = match method {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE"
    } ;
    info!("Attaching: {:6}| {}", method, text);
}

pub fn log_attach_in_out<IN: Debug + Default, OUT: Debug + Serialize + Default >(method: HttpMethod, text: &str) {
    let method: &str = match method {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE"
    } ;
    let in_name = format!("{:?}", IN::default());
    let in_name = in_name.split_whitespace().next().unwrap();
    let out_name = format!("{:?}", OUT::default());
    let out_name = out_name.split_whitespace().next().unwrap();
    info!("Attaching: {:6}| {:25} | In: {} | Out: {}", method, text, in_name, out_name);
}

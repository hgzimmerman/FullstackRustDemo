#[macro_use]
extern crate warp;
extern crate db;
extern crate wire;
extern crate auth;
extern crate uuid;
extern crate identifiers;
extern crate error as joe_error;

extern crate serde;
extern crate serde_json;
extern crate simplelog;

extern crate r2d2;
extern crate r2d2_diesel;
extern crate diesel;
extern crate pool;
#[macro_use]
extern crate lazy_static;

#[macro_use] extern crate log;
extern crate pretty_env_logger;

#[cfg(test)]
extern crate testing_fixtures;
#[cfg(test)]
extern crate testing_common;

mod db_integration;
mod routes;
mod jwt;
mod error;
mod uuid_integration;
mod logging;
mod util;
mod state;


use self::logging::setup_logging;


//lazy_static!(
//    static ref STATE: State = State {
//        pool: db::init_pool(db::DATABASE_URL),
//        secret: RwLock::new(Secret::generate())
//    };
//);

fn main() {
    setup_logging();

    let config = Config::default();
    let state = State::init(config);

    test_2(&state)
        .or(test_2(&state));

    warp::serve(self::routes::routes(&state))
        .run(([127, 0, 0, 1], 3030))
}


use crate::state::Config;
use crate::state::State;
use warp::filters::BoxedFilter;
use pool::PooledConn;
use db::User;
use wire::user::UserResponse;
use crate::error::Error;
use crate::util::convert_vector_and_json;
use warp::Filter;

// TEMPORARY
fn test_2(s: &State) -> BoxedFilter<(impl warp::Reply,)>{
    warp::any()
        .and(s.db.clone())
        .and_then(|conn: PooledConn| {
            db::user::User::get_users(3, &conn)
                .map(convert_vector_and_json::<User, UserResponse>)
                .map_err(Error::convert_and_reject)
        })
        .boxed()
}


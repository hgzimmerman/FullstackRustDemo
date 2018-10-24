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

extern crate diesel;
extern crate pool;

#[macro_use] extern crate log;
//extern crate pretty_env_logger;

#[cfg(test)]
extern crate testing_fixtures;
#[cfg(test)]
extern crate testing_common;

mod routes;
mod error;
mod uuid_integration;
mod logging;
mod util;
mod state;

use self::logging::setup_logging;
use crate::state::Config;
use crate::state::State;

const PORT: u16 = 8001;
fn main() {
    setup_logging();

    let config = Config::default();
    let state = State::init(config);
    warp::serve(self::routes::routes(&state))
        .run(([127, 0, 0, 1], PORT))
}





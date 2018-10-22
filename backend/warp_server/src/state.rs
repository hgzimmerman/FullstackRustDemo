use pool::Pool;
use pool::PooledConn;
use warp::reply::Reply;
use db::Conn;
use warp::Filter;
use warp::http::StatusCode;
use warp::filters::BoxedFilter;
use crate::error::Error;
use db::User;
use wire::user::UserResponse;
use crate::util::convert_vector_and_json;
use std::sync::RwLock;
use auth::Secret;
use db;
use crate::db_integration;


pub struct State {
    pub db: BoxedFilter<(PooledConn,)>,
//    pub secret: RwLock<Secret>,
    pub secret: BoxedFilter<(Secret,)>
}


pub struct Config {
    specified_secret: Option<String>,
    database_url: &'static str
}

impl Default for Config {
    fn default() -> Self {
        Config {
            specified_secret: None,
            database_url: db::DATABASE_URL,
        }
    }
}

use crate::jwt::secret_filter_2;

impl State {
    pub fn init(config: Config) -> State {
        let pool = pool::init_pool(config.database_url);

        // Either randomly generate the secret, or use the user specified text.
        let secret: Secret = if let Some(secret_text) =config.specified_secret {
            Secret::from_user_supplied_string(&secret_text)
        } else {
            Secret::generate()
        };

        State {
            db: db_integration::db_filter(pool),
            secret: secret_filter_2(secret)
        }
    }

    #[cfg(test)]
    pub fn testing_init(pool: Pool, secret: Secret) -> State {
//        let secret = Secret::generat`e();
        State {
            db: db_integration::db_filter(pool),
            secret: secret_filter_2(secret)
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::init(Config::default())
    }
}



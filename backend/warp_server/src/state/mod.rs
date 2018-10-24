use pool::PooledConn;
use warp::filters::BoxedFilter;
use auth::Secret;
use db;


pub mod db_integration;
pub mod jwt;

use self::jwt::secret_filter;


/// State object that should be accessable to most routes.
/// This object will hold references to functions that will allow the production
/// of database connections and secrets used in validating JWTs.
pub struct State {
    pub db: BoxedFilter<(PooledConn,)>,
    pub secret: BoxedFilter<(Secret,)>
}

/// Configuration struct used in constructing the State struct.
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


impl State {
    /// Set up the state.
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
            secret: secret_filter(secret)
        }
    }

    /// An initialization of the State struct that should only be used for testing.
    /// It uses a parameterized Pool, which allows for the same connections used in testing to be provided,
    /// as well as the same secret used to authorize user sign ins.
    #[cfg(test)]
    pub fn testing_init(pool: Pool, secret: Secret) -> State {
        State {
            db: db_integration::db_filter(pool),
            secret: secret_filter(secret)
        }
    }
}

impl Default for State {
    /// Default State created using the default config.
    fn default() -> Self {
        State::init(Config::default())
    }
}



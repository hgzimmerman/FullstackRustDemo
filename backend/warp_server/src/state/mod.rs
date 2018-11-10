pub mod banned_list;
/// This module deals with anything in the server that requires some stateful interaction.
/// This includes DB access, and secret management.
pub mod db_integration;
pub mod jwt;

use self::{
    banned_list::{
        banned_list_filter,
        BannedList,
    },
    jwt::secret_filter,
};
use auth::Secret;
#[cfg(test)]
use pool::Pool;
use pool::PooledConn;
use warp::filters::BoxedFilter;

/// State object that should be accessable to most routes.
/// This object will hold references to functions that will allow the production
/// of database connections and secrets used in validating JWTs.
pub struct State {
    pub db: BoxedFilter<(PooledConn,)>,
    pub secret: BoxedFilter<(Secret,)>,
    pub banned_list: BoxedFilter<(BannedList,)>,
}

/// Configuration struct used in constructing the State struct.
pub struct StateConfig {
    pub specified_secret: Option<String>,
    pub database_url: String,
}

/// By default:
/// * The secret will be randomly generated.
/// * The database URL will point to the default database as defined by an environment variable.
impl Default for StateConfig {
    fn default() -> Self {
        StateConfig {
            specified_secret: None,
            database_url: pool::DATABASE_URL.to_string(),
        }
    }
}

impl State {
    /// Set up the state.
    pub fn init(config: StateConfig) -> State {
        let pool = pool::init_pool(&config.database_url);

        // Either randomly generate the secret, or use the user specified text.
        let secret: Secret = if let Some(secret_text) = config.specified_secret {
            Secret::from_user_supplied_string(&secret_text)
        } else {
            Secret::generate()
        };

        let banned_list: BannedList = BannedList::default();

        State {
            db: db_integration::db_filter(pool),
            secret: secret_filter(secret),
            banned_list: banned_list_filter(banned_list),
        }
    }
}

#[cfg(test)]
impl State {
    /// An initialization of the State struct that should only be used for testing.
    /// It uses a parameterized Pool, which allows for the same connections used in testing to be provided,
    /// as well as the same secret used to authorize user sign ins.
    pub fn testing_init(pool: Pool, secret: Secret) -> State {
        State {
            db: db_integration::db_filter(pool),
            secret: secret_filter(secret),
            banned_list: banned_list_filter(BannedList::default()),
        }
    }
}

impl Default for State {
    /// Default State created using the default config.
    fn default() -> Self {
        State::init(StateConfig::default())
    }
}

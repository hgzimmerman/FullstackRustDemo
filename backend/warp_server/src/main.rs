/// The Warp Server is a reimplementation of the Rocket-based Server.
/// It is more verbose, yet contains far more control over the api composition, as well as providing nice features like Async that Rocket lacks.
/// Instead of using macros and function signatures to define what requests will be serviced, Warp uses function combinators to filter needed data from http requests and server state.

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
extern crate clap;

//extern crate diesel;
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
use crate::state::StateConfig;
use crate::state::State;
use crate::configuration::Config;

const PORT: u16 = 8001;
fn main() {
    setup_logging();

    let (config, state_config): (Config, StateConfig) = configuration::parse_arguments();

    if config.create_admin {
        let _user = configuration::create_admin(&state_config.database_url)
            .expect("Could not create admin user");
        println!("Created Admin user with UserName: Admin, Password: Admin. Please change the password immediately");
    }
    let state = State::init(state_config);

    warp::serve(self::routes::routes(&state))
        .run(([127, 0, 0, 1], PORT))
}

mod configuration {
    use wire::user::NewUserRequest;
    use wire::user::UserRole;
    use db::User;
    use db::user::NewUser;

    use crate::error::Error;
    use crate::state::StateConfig;
    use clap::App;
    use clap::Arg;

    pub struct Config {
        pub create_admin: bool
    }



    pub fn parse_arguments() -> (Config, StateConfig) {
        const CREATE_ADMIN: &'static str = "create_admin";
        const SECRET_KEY: &'static str = "secret_key";

        let matches = App::new("Weekend At Joes Backend")
            .version("0.1.0")
            .author("Henry Zimmerman")
            .about("Monolithic server for the API and frontend of the Weekend at Joes website.")
            .arg(
                Arg::with_name(CREATE_ADMIN)
                    .long("create_admin")
                    .help("Creates an administrator user if one doesn't already exist.")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name(SECRET_KEY)
                    .long("secret")
                    .short("s")
                    .value_name("KEY")
                    .help(
                        "A key string that is used to sign and verify user tokens. By specifying the same key across restarts, user tokens will not be invalidated. If no key is provided, then a random one is generated.",
                    )
                    .takes_value(true),
            )
            .get_matches();

        let create_admin: bool = matches.is_present(CREATE_ADMIN);
        let secret_key: Option<String> = matches.value_of(SECRET_KEY).map(
            String::from,
        );

        let database_url: String = db::DATABASE_URL.to_string();

        let config = Config {
            create_admin,
        };

        let state_config = StateConfig {
            specified_secret: secret_key,
            database_url
        };
        (config, state_config)
    }

    pub fn create_admin(db_url: &str) -> Result<User, Error> {
        let conn = pool::create_single_connection(db_url);
        let mut user: NewUser = NewUserRequest {
            user_name: "Admin".into(),
            display_name: "Admin".into(),
            plaintext_password: "Admin".into(),
        }.into();
        user.roles = vec![
            UserRole::Admin.into(),
            UserRole::Moderator.into(),
            UserRole::Publisher.into(),
            UserRole::Unprivileged.into(),
        ];
        User::create_user(user, &conn)
            .map_err(|_| Error::DatabaseError(Some(String::from("Admin User already exists"))))
    }

}



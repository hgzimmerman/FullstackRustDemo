#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
//#![feature(rand)]
#![feature(test)]
#![recursion_limit="128"]
// #![feature(proc_macro)]


//#![feature(use_extern_macros)]

extern crate db;
extern crate error;
extern crate auth as auth_lib;

extern crate rocket;
//#[macro_use]
extern crate rocket_contrib;
extern crate uuid;
extern crate serde_json;
extern crate serde;
#[macro_use(log)]
extern crate log;
extern crate simplelog;

extern crate test;

extern crate chrono;
extern crate rocket_cors;


extern crate clap;
use clap::{Arg, App};

use rocket::Rocket;

mod routes;
use routes::*;
//mod db;
//mod auth;
//mod error;
//use auth::{Secret, BannedSet};
use db::user::User;
use db::article::Article;
use db::forum::Forum;
use db::thread::Thread;
use db::post::Post;
use db::bucket::Bucket;
use db::question::Question;
use db::answer::Answer;
use db::chat::Chat;
use db::message::Message;

extern crate wire;
extern crate identifiers;


use auth_lib::Secret;
use auth_lib::BannedSet;


use simplelog::{CombinedLogger, TermLogger, WriteLogger, LevelFilter};
use std::fs::File;

//pub use db::schema; // schema internals can be accessed via db::schema::, or via schema::

use rocket::http::Method;
use rocket_cors::AllowedOrigins;

use log::{info, error, warn};

/// Configuration object for rocket initialization.
#[derive(Clone, Debug)]
pub struct Config {
    /// If this is true, an Admin account will be created at app startup if it has not been done so already.
    create_admin: bool,
    /// If a secret key is not provided, one will be randomly generated.
    /// A warning will be emitted if the key is less than 256 characters long.
    /// The server should fail to start if the secret key is less than 128 characters long.
    secret_key: Option<String>,
    /// The url of the database to which the server will connect
    pub db_url: String,
    /// Determines if CORS support is enabled.
    /// CORS is enabled by default in development mode.
    enable_cors: bool
}

impl Default for Config {
    fn default() -> Self {
        Config {
            create_admin: false,
            secret_key: None,
            db_url: db::DATABASE_URL.to_string(),
            enable_cors: false
        }
    }
}


///Initialize the webserver
pub fn init_rocket(config: Config) -> Rocket {

    info!("Initializing rocket with config: {:#?}", config);

    let optionally_attach_cors = |rocket: Rocket| {
        if cfg!(feature = "development") || config.enable_cors {
            warn!("Enabling CORS.");
            let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://[::1]:8000", "http://localhost:8000", "http://localhost:8001"]);
            assert!(failed_origins.is_empty());
            let options = rocket_cors::Cors {
                allowed_origins,
                allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Options, Method::Delete]
                    .into_iter()
                    .map(From::from)
                    .collect(),
                allow_credentials: true,
                ..Default::default()
            };
            rocket.attach(options)
        } else {
            info!("Using default CORS.");
            rocket
        }
    };


    // The secret is used to generate and verify JWTs.
    let secret: Secret = if let Some(ref key) = config.secret_key {
        info!("Using a user-supplied secret key.");
        Secret::from_user_supplied_string(&key)
    } else {
        info!("Generating a random 256 character secret key.");
        Secret::generate()
    };

    // The banned set is a set of user ids that are kept in memory.
    // This is done to prevent banned users with active JWTs from being authenticated, all without every
    // authentication attempt having to check the database.
    let banned_set: BannedSet = BannedSet::new();

    // A pool of database connections. These will be distributed to threads as they service requests.
    let db_pool: pool::Pool = pool::init_pool(&config.db_url);

    // Create a default Admin user if configured to do so.
    if config.create_admin {
        let conn = db::Conn::new(db_pool.get().unwrap());
        match configuration::create_admin(&conn) {
            Ok(user) => warn!("Admin created. You should change its password. The name of the Admin user is: '{}'", user.user_name),
            Err(e) => error!("Failed to create Admin: {:?}", e),
        }
    }

    /// Path should be an &str that starts with a /
    fn format_api(path: &str) -> String {
        String::from("/api") + path
    }

    // Initialize Rocket.
    let rocket: Rocket = rocket::ignite()
        .manage(db_pool)
        .manage(secret)
        .manage(banned_set)
        .mount("/", routes![static_file::files, static_file::js, static_file::wasm, static_file::index])
        .mount(&format_api(User::PATH), User::ROUTES())
        .mount(&format_api(Article::PATH), Article::ROUTES())
        .mount(&format_api(Auth::PATH), Auth::ROUTES())
        .mount(&format_api(Forum::PATH), Forum::ROUTES())
        .mount(&format_api(Thread::PATH), Thread::ROUTES())
        .mount(&format_api(Post::PATH), Post::ROUTES())
        .mount(&format_api(Bucket::PATH), Bucket::ROUTES())
        .mount(&format_api(Question::PATH), Question::ROUTES())
        .mount(&format_api(Answer::PATH), Answer::ROUTES())
        .mount(&format_api(Chat::PATH), Chat::ROUTES())
        .mount(&format_api(Message::PATH), Message::ROUTES())
        .catch(errors![
            static_file::json_404,
            static_file::json_500,
            static_file::json_401,
            static_file::json_403,
        ]);

    optionally_attach_cors(rocket)
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


/// Parses the arguments that the server may use when starting up.
pub fn parse_arguments() -> Config {
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
    let db_url: String = db::DATABASE_URL.to_string();

    Config {
        create_admin,
        secret_key,
        db_url,
        enable_cors: false
    }
}


mod configuration {
    use wire::user::*;
    use db::user::{NewUser, User};
    use db::Conn;
    use error::BackendResult;


    pub fn create_admin(conn: &Conn) -> BackendResult<User> {
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
        User::create_user(user, conn)
    }
}

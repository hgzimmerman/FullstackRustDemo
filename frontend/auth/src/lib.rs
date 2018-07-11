//! This crate is for handling the `/auth/*` routes.
//! This involves account creation and login, but would also encompass password reset.


#[macro_use]
extern crate yew;
#[macro_use]
extern crate yew_router;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate failure;
//extern crate context;
extern crate common;
extern crate wire;
extern crate util;
//extern crate routes;

//pub use context::datatypes;
//pub use context::Context;
//pub use routes::auth::AuthRoute;
//pub use routes::Route;

pub mod login_component;
pub mod create_account_component;
mod requests;

pub use self::login_component::Login;
pub use self::create_account_component::CreateAccount;

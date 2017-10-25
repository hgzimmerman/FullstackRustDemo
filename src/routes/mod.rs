pub mod static_file;
pub mod article;
pub mod user;
pub mod login;


pub use self::article::*;
pub use self::static_file::*;
pub use self::user::*;
pub use self::login::*;

use rocket::Route;

pub trait Routable {
    const ROUTES: &'static Fn() -> Vec<Route>;
    const PATH: &'static str;
}

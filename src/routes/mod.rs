pub mod static_file;
pub mod article;
pub mod user;
pub mod login;
pub mod bucket_questions;
pub mod forum;


pub use self::article::*;
pub use self::static_file::*;
pub use self::user::*;
pub use self::login::*;
pub use self::bucket_questions::*;
pub use self::forum::*;

use rocket::Route;

pub trait Routable {
    const ROUTES: &'static Fn() -> Vec<Route>;
    const PATH: &'static str;
}

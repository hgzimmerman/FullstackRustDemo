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
// these are mostly unused
pub use self::bucket_questions::*;
// pub use self::forum::*;
use rocket::response::{Responder, Response};
use rocket::http::Status;
use rocket::request::Request;

use rocket::Route;

pub trait Routable {
    const ROUTES: &'static Fn() -> Vec<Route>;
    const PATH: &'static str;
}


// Response type to indicate if the backend encountered a database error
#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseError(Option<String>);
impl<'r> Responder<'r> for DatabaseError {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let mut build = Response::build();
        if let Some(error_message) = self.0 {
            build.merge(error_message.respond_to(req)?);
        } else  {
            build.merge("Database Error".to_string().respond_to(req)?);
        }

        build.status(Status::Accepted).ok()
    }
}
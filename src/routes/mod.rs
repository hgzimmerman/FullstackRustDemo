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


#[derive(Debug, Clone, PartialEq)]
pub enum WeekendAtJoesError {
    DatabaseError(Option<String>),
    NotFound{
        type_name: &'static str
    },
    NotAuthorized {
        reason: &'static str
    },
    BadRequest
}

impl<'r> Responder<'r> for WeekendAtJoesError {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let mut build = Response::build();

        use WeekendAtJoesError::*;
        match self {
            DatabaseError(db_error) => {
                if let Some(error_message) = db_error {
                    build.merge(error_message.respond_to(req)?);
                } else  {
                    build.merge("Database Error".to_string().respond_to(req)?);
                }
                build.status(Status::InternalServerError).ok()
            }
            NotFound{type_name} => {
                let err_message = format!("Could not find requested {}", type_name );
                Response::build_from(err_message.respond_to(req)?)
                    .status(Status::NotFound)
                    .ok()
            }
            NotAuthorized {reason} => {
                build.merge(reason.respond_to(req)?)
                    .status(Status::Unauthorized)
                    .ok()
            }
            BadRequest => {
                build.merge("Malformed request".respond_to(req)?)
                    .status(Status::BadRequest)
                    .ok()
            }
        }
    }
}

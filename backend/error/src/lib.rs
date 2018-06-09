extern crate rocket;
extern crate diesel;

use rocket::response::{Responder, Response};
use rocket::http::Status;
use rocket::request::Request;
use diesel::result::Error;

pub type JoeResult<T> = Result<T, WeekendAtJoesError>;

/// A hack that allows the conversion of Result<Vec<T>,E> to Result<Vec<W>,E> as a one liner
pub trait VectorMappable<T> {
    fn map_vec<W>(self) -> JoeResult<Vec<W>>
    where
        W: From<T>;
}

impl<T> VectorMappable<T> for JoeResult<Vec<T>> {
    fn map_vec<W>(self) -> JoeResult<Vec<W>>
    where
        W: From<T>,
    {
        self.map(|vec| {
            vec.into_iter()
                .map(W::from)
                .collect::<Vec<W>>()
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeekendAtJoesError {
    DatabaseError(Option<String>),
    InternalServerError,
    NotFound { type_name: &'static str },
    BadRequest,
    /// The used did not have privalages to access the given method.
    NotAuthorized { reason: &'static str },
    /// The thread being posted to or edited was locked by an admin.
    ThreadLocked,
    /// Used to indicate that the signature does not match the hashed contents + secret
    IllegalToken,
    /// The expired field in the token is in the past
    ExpiredToken,
    /// The request did not have a token.
    MissingToken,
    /// The JWT 'bearer schema' was not followed.
    MalformedToken,
}

pub fn handle_diesel_error(diesel_error: Error, type_name: &'static str) -> WeekendAtJoesError {
    match diesel_error {
        Error::NotFound => WeekendAtJoesError::NotFound { type_name },
        _ => WeekendAtJoesError::DatabaseError(Some(format!("{:?}", diesel_error))), // This gives some insight into what the internal state of the app is. Set this to none when this enters production.
    }
}

pub trait ErrorFormatter {
    fn handle_error(diesel_error: Error) -> WeekendAtJoesError;
}

impl<'r> Responder<'r> for WeekendAtJoesError {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let mut build = Response::build();

        use WeekendAtJoesError::*;
        match self {
            DatabaseError(db_error) => {
                if let Some(error_message) = db_error {
                    build.merge(
                        error_message.respond_to(req)?,
                    );
                } else {
                    build.merge("Database Error"
                        .to_string()
                        .respond_to(req)?);
                }
                build
                    .status(Status::InternalServerError)
                    .ok()
            }
            InternalServerError => {
                build
                    .status(Status::InternalServerError)
                    .ok()
            }
            NotFound { type_name } => {
                let err_message = format!("Could not find requested {}", type_name);
                Response::build_from(err_message.respond_to(req)?)
                    .status(Status::NotFound)
                    .ok()
            }
            NotAuthorized { reason } => {
                build
                    .merge(reason.respond_to(req)?)
                    .status(Status::Forbidden)
                    .ok()
            }
            BadRequest => {
                build
                    .merge("Malformed request".respond_to(req)?)
                    .status(Status::BadRequest)
                    .ok()
            }
            ThreadLocked => {
                build
                    .merge("Thread being operated upon is locked and therefore cant be changed"
                        .respond_to(req)?)
                    .status(Status::MethodNotAllowed)
                    .ok()
            }
            IllegalToken => {
                build
                    .merge("Login token's contents do not match its signature."
                        .respond_to(req)?)
                    .status(Status::Unauthorized)
                    .ok()
            }
            ExpiredToken => {
                build
                    .merge("Login token has expired.".respond_to(
                        req,
                    )?)
                    .status(Status::Unauthorized)
                    .ok()
            }
            MissingToken => {
                build
                    .merge("Login token not supplied.".respond_to(
                        req,
                    )?)
                    .status(Status::Unauthorized)
                    .ok()
            }
            MalformedToken => {
                build
                    .merge("Login token was not specified correctly."
                        .respond_to(req)?)
                    .status(Status::Unauthorized)
                    .ok()
            }
        }
    }
}



pub type LoginResult = Result<String, LoginError>;

/// Logs the user in by validating their password and returning a jwt.


#[derive(Debug)]
pub enum LoginError {
    UsernameDoesNotExist,
    IncorrectPassword,
    AccountLocked,
    PasswordHashingError(&'static str),
    JwtError(JwtError),
    OtherError(&'static str),
}

impl<'a> Responder<'a> for LoginError {
    fn respond_to(self, _: &Request) -> Result<Response<'static>, Status> {
        // TODO: use the string in a custom Status for internal server error
//        info!("User login failed with error: {:?}", &self);
        match self {
            LoginError::IncorrectPassword => Err(Status::Unauthorized),
            LoginError::AccountLocked => Err(Status::Unauthorized),
            LoginError::UsernameDoesNotExist => Err(Status::NotFound),
            LoginError::JwtError(_) => Err(Status::InternalServerError),
            LoginError::PasswordHashingError(_) => Err(Status::InternalServerError),
            LoginError::OtherError(_) => Err(Status::InternalServerError),
        }
    }
}





/// An error that can ocurr in the course of handling JWTs.
#[derive(Debug, Clone)]
pub enum JwtError {
    DecodeError,
    EncodeError,
    DeserializeError,
    SerializeError,
}
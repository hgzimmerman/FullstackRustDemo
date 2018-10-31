extern crate diesel;

use diesel::result::Error;

#[cfg(feature = "rocket_support")]
extern crate rocket;

#[cfg(feature = "warp_support")]
extern crate warp;


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
    DatabaseUnavailable,
    DatabaseError(Option<String>),
    InternalServerError,
    NotFound { type_name: String },
    BadRequest,
    /// The used did not have privileges to access the given method.
    NotAuthorized { reason: &'static str },
    /// The thread being posted to or edited was locked by an admin.
    ThreadImmutable,
    /// Used to indicate that the signature does not match the hashed contents + secret
    IllegalToken,
    /// The expired field in the token is in the past
    ExpiredToken,
    /// The request did not have a token.
    MissingToken,
    /// The JWT 'bearer schema' was not followed.
    MalformedToken,
    /// The user has been banned and therefore can't perform their desired action.
    UserBanned
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


/// An error that can occur in the course of handling JWTs.
#[derive(Debug, Clone)]
pub enum JwtError {
    DecodeError,
    EncodeError,
    DeserializeError,
    SerializeError,
}

pub fn handle_diesel_error(diesel_error: Error, type_name: &'static str) -> WeekendAtJoesError {
    match diesel_error {
        Error::NotFound => WeekendAtJoesError::NotFound { type_name: type_name.to_string() },
        _ => WeekendAtJoesError::DatabaseError(Some(format!("{:?}", diesel_error))), // This gives some insight into what the internal state of the app is. Set this to none when this enters production.
    }
}

pub trait ErrorFormatter {
    fn handle_error(diesel_error: Error) -> WeekendAtJoesError;
}

#[cfg(feature = "rocket_support")]
mod rocket_support {
    use super::*;
    use rocket::response::{Responder, Response};
    use rocket::http::Status;
    use rocket::request::Request;

    impl<'r> Responder<'r> for WeekendAtJoesError {
        fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
            let mut build = Response::build();

            use WeekendAtJoesError::*;
            match self {
                DatabaseUnavailable => {
                    build
                        .merge(
                        "Database Could Not be Reached"
                            .to_string()
                            .respond_to(req)?);
                    build
                        .status(Status::InternalServerError)
                        .ok
                }
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
                ThreadImmutable => {
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
}


#[cfg(feature = "warp_support")]
pub mod warp_support {
    use super::*;
    use std::fmt::Display;
    use std::fmt;
    use std::error::Error as StdError;
    use warp::reject::Rejection;
    use warp::reply::Reply;
    use warp::http::StatusCode;

    impl Display for WeekendAtJoesError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

            let description: String = match self {
                WeekendAtJoesError::DatabaseUnavailable => "Could not acquire a connection to the database, the connection pool may be occupied".to_string(),
                WeekendAtJoesError::DatabaseError(e) => {
                    match e {
                        Some(s) => s.clone(),
                        None => "A problem occurred with the database".to_string()
                    }
                }
                WeekendAtJoesError::IllegalToken => "The provided token is invalid".to_string(),
                WeekendAtJoesError::ExpiredToken => "The provided token has expired, please reauthenticate to acquire a new one".to_string(),
                WeekendAtJoesError::MalformedToken => "The token was not formatted correctly".to_string(),
                WeekendAtJoesError::ThreadImmutable => "The Thread you are trying to interact with has been locked, preventing modification".to_string(),
                WeekendAtJoesError::MissingToken => "The Api route was expecting a JWT token and none was provided. Try logging in.".to_string(),
                WeekendAtJoesError::NotAuthorized {reason} => format!("You are forbidden from accessing this resource. ({})", reason),
                WeekendAtJoesError::UserBanned => "Your account has been banned".to_string(),
                WeekendAtJoesError::BadRequest => "Your request is malformed".to_string(),
                WeekendAtJoesError::InternalServerError => "Internal server error encountered".to_string(),
                WeekendAtJoesError::NotFound {type_name}=> format!("The resource ({})you requested could not be found", type_name),
            };
            write!(f, "{}", description)
        }
    }

    impl StdError for WeekendAtJoesError {
        fn cause(&self) -> Option<&StdError> {
            None
        }
    }

    /// Takes a rejection, which Warp would otherwise handle in its own way, and transform it into
    /// an Ok(Reply) where the status is set to correspond to the provided error.
    ///
    /// This only works if the Rejection is of the custom Error type. Any others will just fall through this unchanged.
    ///
    /// This should be used at the top level of the exposed api.
    pub fn customize_error(err: Rejection) -> Result<impl Reply, Rejection> {
        let mut resp = err.json();

        let cause = match err.into_cause::<crate::WeekendAtJoesError>() {
            Ok(ok) => ok,
            Err(err) => return Err(err)
        };
        match *cause {
            WeekendAtJoesError::DatabaseUnavailable => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
            WeekendAtJoesError::DatabaseError(_) => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
            WeekendAtJoesError::IllegalToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
            WeekendAtJoesError::ExpiredToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
            WeekendAtJoesError::MalformedToken => *resp.status_mut() = StatusCode::UNAUTHORIZED, // Unauthorized is for requests that require authentication and the authentication is out of date or not present
            WeekendAtJoesError::NotAuthorized {..} => *resp.status_mut() = StatusCode::FORBIDDEN, // Forbidden is for requests that will not served due to a lack of privileges
            WeekendAtJoesError::UserBanned => *resp.status_mut() = StatusCode::FORBIDDEN,
            WeekendAtJoesError::BadRequest => *resp.status_mut() = StatusCode::BAD_REQUEST,
            WeekendAtJoesError::NotFound {..}=> *resp.status_mut() = StatusCode::NOT_FOUND,
            WeekendAtJoesError::InternalServerError => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
            WeekendAtJoesError::ThreadImmutable => *resp.status_mut() = StatusCode::BAD_REQUEST,
            WeekendAtJoesError::MissingToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
        }

//        warn!("rewrote error response: {:?}", resp);
        Ok(resp)
    }

}

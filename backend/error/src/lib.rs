extern crate diesel;

use diesel::result::Error as DieselError;

#[cfg(feature = "rocket_support")]
extern crate rocket;

#[cfg(feature = "warp_support")]
extern crate warp;


pub type BackendResult<T> = Result<T, Error>;

/// A hack that allows the conversion of Result<Vec<T>,E> to Result<Vec<W>,E> as a one liner
pub trait VectorMappable<T> {
    fn map_vec<W>(self) -> BackendResult<Vec<W>>
    where
        W: From<T>;
}

impl<T> VectorMappable<T> for BackendResult<Vec<T>> {
    fn map_vec<W>(self) -> BackendResult<Vec<W>>
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
pub enum Error {
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

pub fn handle_diesel_error(diesel_error: DieselError, type_name: &'static str) -> Error {
    match diesel_error {
        DieselError::NotFound => Error::NotFound { type_name: type_name.to_string() },
        _ => Error::DatabaseError(Some(format!("{:?}", diesel_error))), // This gives some insight into what the internal state of the app is. Set this to none when this enters production.
    }
}

pub trait ErrorFormatter {
    fn handle_error(diesel_error: DieselError) -> Error;
}

#[cfg(feature = "rocket_support")]
mod rocket_support {
    use super::*;
    use rocket::{
        response::{Responder, Response},
        http::Status,
        request::Request
    };

    impl<'r> Responder<'r> for Error {
        fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
            let mut build = Response::build();

            use Error::*;
            match self {
                DatabaseUnavailable => {
                    build
                        .merge(
                        "Database Could Not be Reached"
                            .to_string()
                            .respond_to(req)?);
                    build
                        .status(Status::InternalServerError)
                        .ok()
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
                UserBanned => {
                    build
                        .merge("Your account has been banned."
                            .respond_to(req)?)
                        .status(Status::Forbidden)
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

    impl Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

            let description: String = match self {
                Error::DatabaseUnavailable => "Could not acquire a connection to the database, the connection pool may be occupied".to_string(),
                Error::DatabaseError(e) => {
                    match e {
                        Some(s) => s.clone(),
                        None => "A problem occurred with the database".to_string()
                    }
                }
                Error::IllegalToken => "The provided token is invalid".to_string(),
                Error::ExpiredToken => "The provided token has expired, please reauthenticate to acquire a new one".to_string(),
                Error::MalformedToken => "The token was not formatted correctly".to_string(),
                Error::ThreadImmutable => "The Thread you are trying to interact with has been locked, preventing modification".to_string(),
                Error::MissingToken => "The Api route was expecting a JWT token and none was provided. Try logging in.".to_string(),
                Error::NotAuthorized {reason} => format!("You are forbidden from accessing this resource. ({})", reason),
                Error::UserBanned => "Your account has been banned".to_string(),
                Error::BadRequest => "Your request is malformed".to_string(),
                Error::InternalServerError => "Internal server error encountered".to_string(),
                Error::NotFound {type_name}=> format!("The resource ({})you requested could not be found", type_name),
            };
            write!(f, "{}", description)
        }
    }

    impl StdError for Error {
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
        if err.is_not_found() {
            *resp.status_mut() = StatusCode::NOT_FOUND;
            return Ok(resp)
        }

        let cause = match err.find_cause::<Error>() {
            Some(ok) => ok,
            None => return Ok(resp)
        };
        match *cause {
            Error::DatabaseUnavailable => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseError(_) => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
            Error::IllegalToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
            Error::ExpiredToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
            Error::MalformedToken => *resp.status_mut() = StatusCode::UNAUTHORIZED, // Unauthorized is for requests that require authentication and the authentication is out of date or not present
            Error::NotAuthorized {..} => *resp.status_mut() = StatusCode::FORBIDDEN, // Forbidden is for requests that will not served due to a lack of privileges
            Error::UserBanned => *resp.status_mut() = StatusCode::FORBIDDEN,
            Error::BadRequest => *resp.status_mut() = StatusCode::BAD_REQUEST,
            Error::NotFound {..}=> *resp.status_mut() = StatusCode::NOT_FOUND,
            Error::InternalServerError => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
            Error::ThreadImmutable => *resp.status_mut() = StatusCode::BAD_REQUEST,
            Error::MissingToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
        }

//        warn!("rewrote error response: {:?}", resp);
        Ok(resp)
    }

    impl Error {
        pub fn reject<T>(self) -> Result<T, Rejection> {
//        Err(warp::reject::reject().with(self))
            Err(warp::reject::custom(self))
        }

        pub fn simple_reject(self) -> Rejection {
            warp::reject::reject().with(self)
//        warp::reject::custom(self)
        }
    }

}

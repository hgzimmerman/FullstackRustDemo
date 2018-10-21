
use error::WeekendAtJoesError;
use std::error::Error as StdError;
use std::fmt::{self, Display};

use warp::{Rejection, Reply};
use warp::http::StatusCode;
use std::fmt::Debug;
use warp;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    DatabaseUnavailable,
    DatabaseError(Option<String>),
    IllegalToken,
    ExpiredToken,
    MalformedToken,
    NotAuthorized,
    BadRequest,
    NotFound,
    /// Catch all
    InternalServerError,
}


impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::DatabaseUnavailable => "Could not acquire a connection to the database, the connection pool may be occupied",
            Error::DatabaseError(e) => {
                match e {
                    Some(s) => &s,
                    None => "A problem occurred with the database"
                }
            }
            Error::IllegalToken => "The provided token is invalid",
            Error::ExpiredToken => "The provided token has expired, please reauthenticate to acquire a new one",
            Error::MalformedToken => "The token was not formatted correctly",
            Error::NotAuthorized => "You are forbidden from accessing this resource",
            Error::BadRequest => "Your request is malformed",
            Error::InternalServerError => "Internal server error encountered",
            Error::NotFound => "The resource you requested could not be found",
        }
    }

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

    let cause = match err.into_cause::<crate::error::Error>() {
        Ok(ok) => ok,
        Err(err) => return Err(err)
    };
    match *cause {
        Error::DatabaseUnavailable => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
        Error::DatabaseError(_) => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
        Error::IllegalToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
        Error::ExpiredToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
        Error::MalformedToken => *resp.status_mut() = StatusCode::UNAUTHORIZED,
        Error::NotAuthorized => *resp.status_mut() = StatusCode::FORBIDDEN,
        Error::BadRequest => *resp.status_mut() = StatusCode::BAD_REQUEST,
        Error::NotFound => *resp.status_mut() = StatusCode::NOT_FOUND,
        Error::InternalServerError => *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR,
    }

    warn!("rewrote error response: {:?}", resp);
    Ok(resp)
}


impl From<WeekendAtJoesError> for Error {
    fn from(e: WeekendAtJoesError) -> Self {
        use self::WeekendAtJoesError::*;
        match e {
            DatabaseError(s) => Error::DatabaseError(s),
            InternalServerError => Error::InternalServerError,
            NotFound { type_name: _ } => Error::NotFound,
            BadRequest => Error::BadRequest,
            NotAuthorized { reason: _ } => Error::NotAuthorized,
            ThreadImmutable => Error::InternalServerError,
            IllegalToken => Error::IllegalToken,
            ExpiredToken => Error::ExpiredToken,
            MissingToken => Error::MalformedToken,
            MalformedToken => Error::MalformedToken,
        }
    }
}




impl Error {
    /// With the use of or_else(), or just inside of a map() or and_then(),
    /// this allows you to reject a request with a locally defined error.
    pub fn reject<T>(self) -> Result<T, Rejection> {
        Err(warp::reject::reject().with(self))
    }

    pub fn simple_reject(self) -> Rejection {
        warp::reject::reject().with(self)
    }


    pub fn convert_and_reject<T: Into<Self> + Debug>(other_error: T) -> Rejection {
        warn!("Rejecting request due to encountered error: {:?}", other_error);
        let error: Error = other_error.into();
        let rejection = warp::reject::server_error(); // Use server_error() because there is a precedence for the errors and 400 is suprisingly high, preventing the rewrite from working
        let rejection = rejection.with(error);
        warn!("{:?}", rejection);
        rejection
    }
}
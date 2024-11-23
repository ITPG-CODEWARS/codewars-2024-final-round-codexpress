use std::*;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("That is not a valid email address.")]
    InvalidEmailAddressError,

    #[error("The mutex guarding the Sqlite connection was poisoned.")]
    MutexPoisonError,

    #[error("Could not find any user that fits the specified requirements.")]
    UserNotFoundError,

    #[error("UnmanagedStateError: failed retrieving `Users`. You may be missing `.manage(users)` in your app.")]
    UnmanagedStateError,

    #[error("UnauthenticatedError: The operation failed because the client is not authenticated.")]
    UnauthenticatedError,

    #[error("The email \"{0}\" is not registered. Try signing up first.")]
    EmailDoesNotExist(String),

    #[error("That email address already exists. Try logging in.")]
    EmailAlreadyExists,

    #[error("Incorrect email or password")]
    UnauthorizedError,

    #[error("{0}")]
    FormValidationError(#[from] validator::ValidationError),

    #[error("FormValidationErrors: {0}")]
    FormValidationErrors(#[from] validator::ValidationErrors),

    #[error("SqlxError: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Argon2ParsingError: {0}")]
    Argon2ParsingError(#[from] argon2::Error),

    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
}

/*****  CONVERSIONS  *****/
use std::sync::PoisonError;
impl<T> From<PoisonError<T>> for Error {
    fn from(_error: PoisonError<T>) -> Error {
        Error::MutexPoisonError
    }
}

use self::Error::*;
impl Error {
    fn message(&self) -> String {
        match self {
            InvalidEmailAddressError
            | EmailAlreadyExists
            | UnauthorizedError
            | UserNotFoundError => format!("{}", self),
            FormValidationErrors(source) => source
                .field_errors()
                .into_iter()
                .map(|(_, error)| error)
                .map(IntoIterator::into_iter)
                .map(|errs| errs.map(|err| &err.code).fold(String::new(), |a, b| a + b))
                .fold(String::new(), |a, b| a + &b),
            #[cfg(debug_assertions)]
            e => return format!("{}", e),
            #[allow(unreachable_patterns)]
            _ => "undefined".into(),
        }
    }
}

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde_json::*;
use std::io::Cursor;

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let payload = to_string(&json!({
            "status": "error",
            "message": self.message(),
        }))
        .unwrap();
        Response::build()
            .sized_body(payload.len(), Cursor::new(payload))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

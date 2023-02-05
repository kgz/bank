use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // #[error("wrong credentials")]
    // WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    // #[error("no permission")]
    // NoPermissionError,
    // #[error("404")]
    // NotFound,
    #[error("internal server error")]
    InternalServerError,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}


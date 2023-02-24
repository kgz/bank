use regex::Regex;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
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
    _InternalServerError,
    #[error("invalid data")]
    InvalidData,
    #[error("Form Validation Error: {0}")]
    FormValidationError(String),
    #[error("{0}")]
    Custom(String),
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}

pub fn format_serd_error(e: String) -> Error {
    // replace regex (at line \d+ column \d+) with nothing
    let message = format!("{}", e).replace("(\\sat line \\d+ column \\d+)", "");
    let re = Regex::new("(\\sat line \\d+ column \\d+)").unwrap();
    let message = re.replace_all(&message, "");

    println!("message: {:?}", message);
    Error::FormValidationError(message.to_string())

}


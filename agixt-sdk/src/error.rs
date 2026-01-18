//! Error types for the AGiXT SDK.

use std::fmt;

/// Error types for AGiXT SDK operations.
#[derive(Debug)]
pub enum Error {
    /// Error from the HTTP client
    RequestError(reqwest::Error),
    /// Error parsing JSON
    JsonError(serde_json::Error),
    /// Error from the AGiXT API
    ApiError {
        status: u16,
        message: String,
    },
    /// Error with authentication
    AuthError(String),
    /// Invalid input parameters
    InvalidInput(String),
    /// Resource not found
    NotFound(String),
    /// Generic error for other cases
    Other(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RequestError(e) => write!(f, "Request error: {}", e),
            Error::JsonError(e) => write!(f, "JSON error: {}", e),
            Error::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            Error::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::RequestError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

/// Result type alias using the AGiXT Error type.
pub type Result<T> = std::result::Result<T, Error>;

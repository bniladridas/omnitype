//! Error types for the omnitype crate.

use thiserror::Error;

/// A type alias for `Result<T, Error>`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The main error type for the omnitype crate.
#[derive(Error, Debug)]
pub enum Error {
    /// I/O related errors.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Parser related errors.
    #[error("Parser error: {0}")]
    Parser(String),

    /// Type checking related errors.
    #[error("Type error: {0}")]
    Type(String),

    /// Invalid argument errors.
    #[error("Invalid argument: {0}")]
    Argument(String),

    /// Feature not implemented yet.
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Other miscellaneous errors.
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Creates a new parser error.
    pub fn parser_error(msg: impl Into<String>) -> Self {
        Self::Parser(msg.into())
    }

    /// Creates a new type error.
    pub fn type_error(msg: impl Into<String>) -> Self {
        Self::Type(msg.into())
    }

    /// Creates a new argument error.
    pub fn argument_error(msg: impl Into<String>) -> Self {
        Self::Argument(msg.into())
    }

    /// Creates a new not implemented error.
    pub fn not_implemented(feature: impl Into<String>) -> Self {
        Self::NotImplemented(feature.into())
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Other(s.to_string())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}

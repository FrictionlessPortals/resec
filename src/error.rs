//! Custom Error Implementation for ``resec``.
//!
//! This custom error implementation allows the user to
//! quickly filter between errors allowing for fast error checking.

use std::io;
use thiserror::Error;

/// Generic Result for the library.
pub(crate) type SecResult<T, E = SecError> = Result<T, E>;

#[derive(Error, Debug)]
pub enum SecError {
    #[error("IO failure")]
    Io(#[from] io::Error),
    #[error("Reqwest failure")]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to retrieve {0} value")]
    Value(&'static str),
}

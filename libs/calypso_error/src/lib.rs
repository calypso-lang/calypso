//! Error-handling utilities and types for use in
//! [Calypso](https://github.com/calypso-lang/calypso).
#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso_error/index.html")]
#![warn(clippy::pedantic)]

use thiserror::Error;

/// The Calypso error type.
#[derive(Error, Debug)]
pub enum CalError {
    /// IO errors
    #[error("i/o error")]
    Io(#[from] std::io::Error),
    /// UTF-8 decoding errors
    #[error("utf-8 decoding error")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    /// Any other error, using [`anyhow`]
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// A handy alias for [`Result<T, CalError>`], genericized over `T`.
pub type CalResult<T> = Result<T, CalError>;

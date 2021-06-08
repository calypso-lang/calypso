//! Error-handling utilities and types for use in
//! [Calypso](https://github.com/calypso-lang/calypso).
#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso_error/index.html")]
#![warn(clippy::pedantic)]

use std::fmt::{Debug, Display};

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
    /// Formatting errors
    #[error("formatting error")]
    Fmt(#[from] std::fmt::Error),
    /// Any other error, using [`anyhow`]
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl CalError {
    pub fn try_downcast<E>(self) -> Result<E, Self>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if let CalError::Other(err) = self {
            let x = err.downcast()?;
            Ok(x)
        } else {
            Err(self)
        }
    }

    pub fn try_downcast_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if let CalError::Other(err) = self {
            err.downcast_ref()
        } else {
            None
        }
    }

    pub fn try_downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if let CalError::Other(err) = self {
            err.downcast_mut()
        } else {
            None
        }
    }

    pub fn other_is<E>(&self) -> bool
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        if let CalError::Other(err) = self {
            err.is::<E>()
        } else {
            false
        }
    }
}

/// A handy alias for [`Result<T, CalError>`], genericized over `T`.
pub type CalResult<T> = Result<T, CalError>;

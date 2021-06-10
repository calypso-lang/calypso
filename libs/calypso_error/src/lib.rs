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
    /// Try to downcast the error into a concrete type, if the error is a
    /// [`CalError::Other`].
    ///
    /// # Errors
    ///
    /// `self` is returned if the error could not be downcast.
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

    /// Try to downcast a reference to the error into a reference to a concrete
    /// type, if the error is a [`CalError::Other`].
    #[must_use]
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

    /// Try to downcast a mutable reference to the error into a mutable
    /// reference to a concrete type, if the error is a [`CalError::Other`].
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
}

/// A handy alias for [`Result<T, CalError>`], genericized over `T`.
pub type CalResult<T> = Result<T, CalError>;

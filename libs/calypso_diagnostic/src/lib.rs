//! Compiler diagnostics used in Calypso for error reporting.

#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso_diagnostic/index.html")]
#![warn(clippy::pedantic)]

pub use calypso_error;
pub use codespan_reporting as reporting;

pub mod diagnostic;
pub mod error;
pub mod report;
pub mod types;

/// Re-exports used for diagnostics.
pub mod prelude {
    pub use super::calypso_error::{CalError, CalResult};
    pub use super::diagnostic;
    pub use super::err;
    pub use super::error::DiagnosticError;
}

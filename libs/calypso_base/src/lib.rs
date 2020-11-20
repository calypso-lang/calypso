#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso_base/index.html")]

//! Base types and utilities for Calypso that don't require any other Calypso packages

/// Common macros for ease of use in other parts of the program.
mod macros;
/// [`Span`](span::Span) and [`Spanned`](span::Spanned) types.
pub mod span;
pub use macros::*;
/// Utilities for handling streams of data
pub mod streams;

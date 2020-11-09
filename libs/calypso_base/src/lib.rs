//! Base types and utilities for Calypso that don't require any other Calypso packages

/// Common macros for ease of use in other parts of the program.
mod macros;
/// [`Span`](span::Span) and [`Spanned`](span::Spanned) types.
pub mod span;
pub use macros::*;
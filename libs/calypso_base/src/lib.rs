//! Base types and utilities for Calypso that don't require any other Calypso packages
#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso_base/index.html")]
#![warn(clippy::pedantic)]

/// [`Span`](span::Span) and [`Spanned`](span::Spanned) types.
#[cfg(feature = "span")]
pub mod span;
/// Utilities for handling streams of data
#[cfg(feature = "stream")]
pub mod streams;
/// Utilities for interned strings (symbols)
#[cfg(feature = "symbol")]
pub mod symbol;

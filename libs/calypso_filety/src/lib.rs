//! # `calypso_filety`
//!
//! Binary file type definitions, parsers, high-level interfaces, and more for
//! file types used in [Calypso](https://github.com/calypso-lang/calypso).
//!
//! File types currently included are
//! - Calypso Container File Format (CCFF). For more information, see
//!   [the "spec"](https://github.com/calypso-lang/calypso/blob/main/libs/calypso_filety/ccff.md).
//!   Interfaces are located in the module `ccff`, with a high-level interface
//!   in `ccff::hl` and a lower-level (as the binary format with bincode)
//!   interface in `ccff::ll`.
#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso_filety/index.html")]
#![warn(clippy::pedantic)]

// todo(@ThePuzzlemaker: frame|filety):
//   reimplement using nom and improving the binary format. this will probably
//     cause backwards-incompatibility, but it will allow for better space
//     efficiency.

/// Calypso Container File Format. See the [module-level docs](self) for more information.
pub mod ccff;

pub mod ccff_new;

#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

pub mod arena;
pub mod ctxt;
pub mod diag;
pub mod symbol;
pub mod syntax;

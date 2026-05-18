#![warn(clippy::pedantic)]
#![allow(
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

pub mod arena;
pub mod symbol;
pub mod syntax;

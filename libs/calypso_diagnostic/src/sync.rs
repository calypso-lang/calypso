use crate::report::Report;

#[derive(Clone, Debug)]
/// A lexer/parser synchronization state.
pub enum SyncState<T> {
    /// The lexer/parser had to synchronise,
    /// so it was able to produce `T` but also
    /// produced a diagnostic report.
    Syncd(T, Report),
    /// The lexer/parser didn't have to
    /// synchronize, and it also succeded at
    /// producing `T`.
    Good(T),
    /// The lexer/parser went into panic mode
    /// and may have produced some `T` before
    /// panicking and producing a diagnostic
    /// report.
    Panic(Option<T>, Report),
}

impl<T> SyncState<T> {
    pub fn unwrap_good(self) -> T {
        use SyncState::*;
        match self {
            Good(val) => val,
            Panic(..) => {
                panic!("Called `SyncState::unwrap_good` on a `Panic` value.")
            }
            Syncd(..) => {
                panic!("Called `SyncState::unwrap_good` on a `Syncd` value.")
            }
        }
    }
}

use super::prelude::CalResult;

pub type CalResultSync<T> = CalResult<SyncState<T>>;

pub mod prelude {
    pub use super::CalResultSync;
    pub use super::SyncState::*;
    pub use crate::report::Report;
}

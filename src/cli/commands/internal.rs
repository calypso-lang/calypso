use std::sync::Arc;

use calypso::{ctxt::GlobalCtxt, error::CalResult};

use super::internal::unpretty::unpretty;
use crate::cli::InternalCmd;

pub mod unpretty;

#[allow(clippy::single_match)]
pub fn internal(gcx: &Arc<GlobalCtxt>, cmd: &InternalCmd) -> CalResult<()> {
    match cmd {
        InternalCmd::Panic => panic!("Intentional panic to test ICE handling, please ignore."),
        InternalCmd::Unpretty {
            format,
            input,
            repl,
        } => unpretty(gcx, *format, input.as_ref(), *repl),
    }
}

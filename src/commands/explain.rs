use std::sync::Arc;

use clap::ArgMatches;

use calypso_common::gcx::GlobalCtxt;
use calypso_diagnostic::prelude::*;
use calypso_diagnostic::types;

pub fn explain(gcx: &Arc<GlobalCtxt>, matches: &ArgMatches) -> CalResult<()> {
    let error_code = matches.value_of("EXXXX").unwrap();
    if let Some(diagnostic) = types::DIAGNOSTICS.get(error_code) {
        if let Some(information) = diagnostic.1 {
            print!("{}", information);
        } else {
            let mut emit = gcx.emit.lock();
            let err = &mut emit.err;

            err.error(
                None,
                "no extended information for error code",
                Some(&format!("`{}`", error_code)),
            )?;
        }
    } else {
        let mut emit = gcx.emit.lock();
        let err = &mut emit.err;

        err.error(
            None,
            "error code is invalid",
            Some(&format!("`{}`", error_code)),
        )?;
    }
    Ok(())
}

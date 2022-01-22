use std::sync::Arc;

use calypso_common::gcx::GlobalCtxt;
use calypso_diagnostic::prelude::*;
use calypso_diagnostic::types;

pub fn explain(gcx: &Arc<GlobalCtxt>, error_code: &str) -> CalResult<()> {
    if let Some(diagnostic) = types::DIAGNOSTICS.get(error_code) {
        if let Some(information) = diagnostic {
            print!("{}", information);
        } else {
            let mut emit = gcx.emit.write();
            let err = &mut emit.err;

            err.error(
                None,
                "no extended information for error code",
                Some(&format!("`{}`", error_code)),
            )?
            .flush()?;
        }
    } else {
        let mut emit = gcx.emit.write();
        let err = &mut emit.err;

        err.error(
            None,
            "error code is invalid",
            Some(&format!("`{}`", error_code)),
        )?
        .flush()?;
    }
    Ok(())
}

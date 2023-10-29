use std::sync::Arc;

use calypso::{ctxt::GlobalCtxt, diagnostic::types, error::CalResult};

pub fn explain(gcx: &Arc<GlobalCtxt>, error_code: &str) -> CalResult<()> {
    if let Some(information) = types::DIAGNOSTICS.get(error_code) {
        print!("{information}");
    } else {
        let mut emit = gcx.emit.write();
        let err = &mut emit.err;

        err.error(
            None,
            "no extended information for error code",
            Some(&format!("`{error_code}`")),
        )?
        .flush()?;
    }
    Ok(())
}

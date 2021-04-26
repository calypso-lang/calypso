use std::sync::Arc;

use clap::ArgMatches;

use calypso_base::session::BaseSession;
use calypso_base::ui;
use calypso_diagnostic::types;

pub fn explain(sess: Arc<BaseSession>, matches: &ArgMatches) {
    let error_code = matches.value_of("EXXXX").unwrap();
    if let Some(diagnostic) = types::DIAGNOSTICS.get(error_code) {
        if let Some(information) = diagnostic.1 {
            print!("{}", information);
        } else {
            ui::error_to(
                &sess.stderr,
                None,
                "no extended information for error code",
                Some(&format!("`{}`", error_code)),
            )
            .unwrap();
        }
    } else {
        ui::error_to(
            &sess.stderr,
            None,
            "error code is invalid",
            Some(&format!("`{}`", error_code)),
        )
        .unwrap();
    }
}

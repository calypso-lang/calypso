use clap::ArgMatches;

use crate::messages::error;

use calypso_diagnostic::types;

pub fn explain(matches: &ArgMatches) {
    let error_code = matches.value_of("EXXXX").unwrap();
    if let Some(diagnostic) = types::DIAGNOSTICS.get(error_code) {
        if let Some(information) = diagnostic.1 {
            print!("{}", information);
        } else {
            error(format!(
                "no extended information for error code `{}`",
                error_code
            ));
        }
    } else {
        error(format!("error code `{}` is invalid", error_code));
    }
}

// use calypso_parsing::raw::{pest, CalypsoParser, Rule};

use clap::ArgMatches;

// use pest::Parser;

// use std::fs;
use std::path::Path;

use crate::messages::error;

pub fn debug(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("dump") {
        dump(matches);
    }
}

pub fn dump(matches: &ArgMatches) {
    let path = Path::new(matches.value_of("INPUT").unwrap());
    if !path.exists() {
        error(format!("file does not exist: `{}`", path.display()));
        return;
    }

    /*let contents = match fs::read_to_string(&path) {
            Ok(v) => v,
            Err(err) => {
                error(format!(
                    "while reading file `{}`: `{}`",
                    path.display(),
                    err
                ));
                return;
            }
        };

        let raw = matches.is_present("raw");
        let pretty = matches.is_present("pretty");
    */

    /*if raw {
        /*let parsed = match CalypsoParser::parse(Rule::file, &contents) {
            Ok(v) => v,
            Err(err) => {
                error(format!(
                    "syntax error while parsing file `{}`:\n{}",
                    path.display(),
                    err
                ));
                return;
            }
        };
        if pretty {
            println!("{:#?}", parsed);
        } else {
            println!("{:?}", parsed);
        }*/
    } else {
        unimplemented!();
    }*/
    unimplemented!();
}

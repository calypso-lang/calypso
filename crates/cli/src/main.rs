#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate, clippy::return_self_not_must_use)]

use std::{io::Read, path::PathBuf};

use compiler::{symbol::Symbol};
use clap::{Parser, Subcommand};
use color_eyre::eyre::{self, eyre};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Commands used for debugging Calypso's internals and implementation.
    #[clap(visible_alias = "dbg")]
    Debug {
        #[clap(subcommand)]
        command: DebugCommand,
    },
}

#[derive(Debug, Subcommand)]
enum DebugCommand {
    /// Run the lexer on a file or stdin.
    Lex {
        /// The file to run the lexer on, or `-` for stdin.
        #[clap(value_parser = FileInput::parse)]
        file: FileInput,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum FileInput {
    File(PathBuf),
    Stdin,
}

impl FileInput {
    #[allow(clippy::unnecessary_wraps)]
    fn parse(s: &str) -> Result<FileInput, clap::Error> {
        if s == "-" {
            Ok(FileInput::Stdin)
        } else {
            Ok(FileInput::File(PathBuf::from(s)))
        }
    }

    fn name(&self) -> Option<&str> {
        match self {
            FileInput::File(path) => Some(path.to_str().unwrap()),
            FileInput::Stdin => None,
        }
    }

    fn read_to_string(&self) -> eyre::Result<String> {
        match self {
            FileInput::File(path) => Ok(std::fs::read_to_string(path)?),
            FileInput::Stdin => {
                let mut s = String::new();
                std::io::stdin().read_to_string(&mut s)?;
                Ok(s)
            }
        }
    }
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    match args.command {
        Command::Debug { command } => debug(command),
    }
}

fn debug(command: DebugCommand) -> eyre::Result<()> {
    match command {
        DebugCommand::Lex { file } => {
            let source = file.read_to_string()?;
            let tokens = lexer::tokens(&source, Symbol::intern(file.name().unwrap_or("<stdin>")));
            for res in tokens {
                let (span, tok) = res.map_err(|e| eyre!("{:#?}", e))?;
                println!("{}..{}: {:?}", span.lo(), span.hi(), tok);
            }
        }
    }

    Ok(())
}

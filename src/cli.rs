use clap::StructOpt;
use std::{
    fmt::{self, Display},
    path::PathBuf,
};

use calypso_base::ui::{self, atty::Stream, termcolor::ColorChoice};

#[derive(Debug, StructOpt)]
pub struct Args {
    /// Set how color is displayed, if at all. By default this is set to
    /// `auto`.
    ///
    /// Possible values:
    ///
    /// - `always`: Always use color, even if stdout/stderr is not a TTY
    ///
    /// - `ansi`: Always use color, using ANSI escape codes, even if
    /// stdout/stderr is not a TTY or does not support them.
    ///
    /// - `auto`: Use color if stdout/stderr is a TTY, don't if it is not.
    ///
    /// - `never`: Never use color, even if stdout/stderr is a TTY.
    #[structopt(
        long,
        parse(from_str = parse_color_prefs),
        possible_values = &[
            "always",
            "ansi",
            "auto",
            "never"
        ],
        default_value = "auto"
    )]
    pub color: (ColorChoice, ColorChoice),

    /// The logging filter to use.
    ///
    /// See tracing-subscriber's EnvFilter type for an explanation of the format:
    ///
    /// https://docs.rs/tracing-subscriber/*/tracing_subscriber/filter/struct.EnvFilter.html
    #[structopt(long, env = "CALYPSO_LOG")]
    pub log: Option<String>,

    /// The logging format to use.
    ///
    /// Formats available:
    ///
    /// - `pretty`: Overly verbose, but good for human consumption.
    ///
    /// - `compact`: Less verbose, but still readable.
    ///
    /// - `json`: Intended for machine interpretation.
    /// (see https://docs.rs/tracing-subscriber/*/tracing_subscriber/fmt/format/struct.Json.html)
    #[structopt(
        long,
        env = "CALYPSO_LOG_FORMAT",
        possible_values = &[
            "pretty",
            "compact",
            "json"
        ],
        default_value = "compact",
        parse(from_str = parse_log_format)
    )]
    pub log_format: LogFormat,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Explain an error that has detailed information on troubleshooting.
    #[structopt(visible_aliases = &["expl", "exp", "ex"])]
    Explain {
        /// The error to get information for. This must be the error code of
        /// the error, which is of the form `EXXXX` (e.g. E0591).
        #[structopt(name = "EXXXX")]
        ecode: String,
    },
    /// Commands used for debugging Calypso's internals and implementation.
    #[structopt(visible_alias = "int")]
    Internal {
        #[structopt(subcommand)]
        cmd: InternalCmd,
    },
}

#[derive(Debug, StructOpt)]
pub enum InternalCmd {
    /// Show information about the environment where this executable was built.
    #[structopt(visible_aliases = &["bi", "buildinfo"])]
    BuildInfo,
    /// Intentionally panic in order to test out handling of ICEs (internal
    /// compiler errors).
    Panic,
    /// Run analyses on Calypso source files and return the result in an
    /// "unpretty" format, e.g. AST (abstract syntax tree) or token list.
    #[structopt(visible_alias = "up")]
    Unpretty {
        /// Use a REPL-like interface when using standard input. This does not
        /// affect behaviour when using file input.
        #[structopt(short, long)]
        repl: bool,
        /// The "unpretty" format to output.
        ///
        /// Current formats:
        ///
        /// - `toks`: Token list
        ///
        /// - `ast`: Abstract syntax tree (AST)
        #[structopt(possible_values = &[
            "toks",
            "ast"
        ], parse(from_str = parse_unpretty))]
        format: UnprettyFormat,
        /// The input file to run transformations on. Use the file name `-`
        /// (without backticks) to use standard input.
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
}

#[derive(Copy, Clone, Debug)]
pub enum UnprettyFormat {
    TokenList,
    Ast,
}

impl Display for UnprettyFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UnprettyFormat::Ast => write!(f, "ast"),
            UnprettyFormat::TokenList => write!(f, "toks"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LogFormat {
    Pretty,
    Compat,
    Json,
}

fn parse_color_prefs(s: &str) -> (ColorChoice, ColorChoice) {
    (
        ui::parse_color_pref(s, Stream::Stdout),
        ui::parse_color_pref(s, Stream::Stderr),
    )
}

fn parse_unpretty(s: &str) -> UnprettyFormat {
    match s {
        "toks" => UnprettyFormat::TokenList,
        "ast" => UnprettyFormat::Ast,
        _ => unreachable!(),
    }
}

fn parse_log_format(s: &str) -> LogFormat {
    match s {
        "pretty" => LogFormat::Pretty,
        "compact" => LogFormat::Compat,
        "json" => LogFormat::Json,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        use clap::IntoApp;
        Args::into_app().debug_assert()
    }
}

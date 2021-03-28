#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso/index.html")]
#![warn(clippy::pedantic)]

use clap::{load_yaml, App};

mod commands;
mod messages;

#[cfg(feature = "enable_mimalloc")]
use mimalloc::MiMalloc;
use tracing_subscriber::EnvFilter;

#[cfg(feature = "enable_mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// use messages::{error, info, warn};

fn main() {
    let yaml = load_yaml!("data/cli-en.yml");
    let matches = App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_env("CALYPSO_LOG"))
        .pretty()
        .init();

    match matches.subcommand() {
        ("internal", Some(matches)) => commands::internal(matches),
        ("explain", Some(matches)) => commands::explain(matches),
        _ => unreachable!(),
    }
}

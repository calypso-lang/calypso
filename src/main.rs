#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso/index.html")]
#![warn(clippy::pedantic)]

use std::panic;
use std::sync::Arc;

use clap::{load_yaml, App};
use once_cell::sync::OnceCell;
use tracing_subscriber::EnvFilter;

use calypso_base::ui::{self, atty::Stream, Emitters};
use calypso_common::parking_lot::Mutex;
use calypso_common::{gcx::GlobalCtxt, parking_lot::RwLock};
use calypso_diagnostic::prelude::*;
use calypso_diagnostic::{diagnostic::SourceMgr, report::GlobalReportingCtxt};

mod buildinfo;
mod commands;

use buildinfo::BUILD_INFO;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
#[cfg_attr(feature = "mimalloc", global_allocator)]
static GLOBAL: MiMalloc = MiMalloc;

static DEFAULT_HOOK: OnceCell<Box<dyn Fn(&panic::PanicInfo<'_>) + Sync + Send + 'static>> =
    OnceCell::new();

const BUG_REPORT_URL: &str = "https://github.com/calypso-lang/calypso/issues/new\
    ?assignees=&labels=C-bug&template=bug-report.md&title=bug%3A+";

fn init_panic_hook(gcx: &Arc<GlobalCtxt>) {
    // This is dumb but borrowck really wants me to do it this way. Luckily the
    // remaining useless `Arc`s will just be dropped, and this is just init
    // code.
    let gcx = Arc::clone(&gcx);
    DEFAULT_HOOK.get_or_init(|| {
        let gcx = Arc::clone(&gcx);
        let hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            let gcx = Arc::clone(&gcx);
            report_ice(&*gcx, info, BUG_REPORT_URL).unwrap()
        }));
        hook
    });
}

fn report_ice(gcx: &GlobalCtxt, info: &panic::PanicInfo<'_>, report_url: &str) -> CalResult<()> {
    // Invoke the default handler, which prints the actual panic message and
    // optionally a backtrace
    DEFAULT_HOOK.get().unwrap()(info);

    // Separate the output with an empty line
    eprintln!();

    let mut emit = gcx.emit.lock();
    let err = &mut emit.err;

    err.error(
        None,
        "the compiler unexpectedly crashed. this is a bug.",
        None,
    )?
    .note("we would appreciate a bug report at", Some(report_url))?
    .note(
        "build information",
        Some(&format!(
            "calypso {} ({}) running on {}",
            BUILD_INFO.version, BUILD_INFO.git_commit, BUILD_INFO.cargo_target_triple
        )),
    )?
    .note(
        "for further information, run",
        Some("`calypso internal buildinfo`"),
    )?
    .flush()?;

    Ok(())
}

fn main() {
    let yaml = load_yaml!("data/cli-en.yml");
    let matches = App::from_yaml(yaml)
        .version(BUILD_INFO.version)
        .get_matches();

    let color_pref = matches.value_of("color").unwrap();
    let color_pref_stdout = ui::parse_color_pref(color_pref, Stream::Stdout);
    let color_pref_stderr = ui::parse_color_pref(color_pref, Stream::Stderr);

    let gcx = Arc::new(GlobalCtxt {
        emit: Mutex::new(Emitters::new(color_pref_stdout, color_pref_stderr)),
        grcx: RwLock::new(GlobalReportingCtxt::new()),
        sourcemgr: RwLock::new(SourceMgr::new()),
    });

    init_panic_hook(&gcx);
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_env("CALYPSO_LOG"))
        .pretty()
        .init();

    match matches.subcommand() {
        ("internal", Some(matches)) => commands::internal(&gcx, matches).unwrap(),
        ("explain", Some(matches)) => commands::explain(&gcx, matches).unwrap(),
        _ => unreachable!(),
    }
}

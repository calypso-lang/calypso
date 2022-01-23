#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso/index.html")]
#![warn(clippy::pedantic)]

use std::panic;
use std::sync::Arc;

use clap::StructOpt;
use once_cell::sync::OnceCell;
use tracing_subscriber::EnvFilter;

use calypso_base::ui::Emitters;
use calypso_common::{gcx::GlobalCtxt, parking_lot::RwLock};
use calypso_diagnostic::prelude::*;
use calypso_diagnostic::{diagnostic::SourceMgr, report::GlobalReportingCtxt};

mod buildinfo;
mod cli;
mod commands;

use buildinfo::BUILD_INFO;
use cli::{Args, Command, LogFormat};

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
    let gcx = Arc::clone(gcx);
    DEFAULT_HOOK.get_or_init(|| {
        let gcx = Arc::clone(&gcx);
        let hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            let gcx = Arc::clone(&gcx);
            report_ice(&*gcx, info, BUG_REPORT_URL).unwrap();
        }));
        hook
    });
}

fn report_ice(gcx: &GlobalCtxt, info: &panic::PanicInfo<'_>, report_url: &str) -> CalResult<()> {
    // Invoke the default handler, which prints the actual panic message and
    // optionally a backtrace
    DEFAULT_HOOK.get().unwrap()(info);

    gcx.emit
        .write()
        .err
        .newline()?
        .error(
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
            Some("`calypso internal build-info`"),
        )?
        .flush()?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    let gcx = Arc::new(GlobalCtxt {
        emit: RwLock::new(Emitters::new(args.color.0, args.color.1)),
        grcx: RwLock::new(GlobalReportingCtxt::new()),
        sourcemgr: RwLock::new(SourceMgr::new()),
    });

    init_panic_hook(&gcx);
    let mut trace = tracing_subscriber::fmt::fmt().with_env_filter(EnvFilter::default());

    if let Some(log) = args.log {
        trace = trace.with_env_filter(EnvFilter::new(log));
    }

    match args.log_format {
        LogFormat::Pretty => {
            trace.pretty().init();
        }
        LogFormat::Compat => {
            trace.compact().init();
        }
        LogFormat::Json => {
            trace.json().init();
        }
    }

    let res = match args.cmd {
        Command::Explain { ecode } => commands::explain(&gcx, &ecode),
        Command::Internal { cmd } => commands::internal(&gcx, &cmd),
    };
    if let Err(e) = res {
        gcx.emit
            .write()
            .err
            .error(None, &e.to_string(), None)
            .unwrap()
            .flush()
            .unwrap();
    }
}

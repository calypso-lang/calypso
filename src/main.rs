#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso/index.html")]
#![warn(clippy::pedantic)]

use std::panic;
use std::sync::Arc;

use calypso::error::CalResult;
use calypso::{ctxt::GlobalArenas, diagnostic::DiagReportCtxt};
use clap::StructOpt;
use parking_lot::RwLock;
use tracing_subscriber::EnvFilter;

use calypso::{ctxt::GlobalCtxt, ui::Emitters};

mod cli;

use cli::{Args, Command, LogFormat};

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
#[cfg_attr(feature = "mimalloc", global_allocator)]
static GLOBAL: MiMalloc = MiMalloc;

const BUG_REPORT_URL: &str = "https://glithub.com/calypso-lang/calypso/issues/new\
    ?assignees=&labels=C-bug&template=bug-report.md&title=bug%3A+";

fn init_panic_hook(gcx: &Arc<GlobalCtxt>) {
    let gcx = Arc::clone(gcx);
    let hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        report_ice(&hook, &gcx, info, BUG_REPORT_URL).unwrap();
    }));
}

type PanicHook = Box<dyn Fn(&panic::PanicInfo<'_>) + Send + Sync + 'static>;
fn report_ice(
    hook: &PanicHook,
    gcx: &Arc<GlobalCtxt>,
    info: &panic::PanicInfo<'_>,
    report_url: &str,
) -> CalResult<()> {
    // Invoke the default handler, which prints the actual panic message and
    // optionally a backtrace
    hook(info);

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
        .flush()?;

    Ok(())
}

fn main() {
    let args = Args::parse();

    let gcx = Arc::new(GlobalCtxt {
        emit: RwLock::new(Emitters::new(args.color.0, args.color.1)),
        diag: RwLock::new(DiagReportCtxt::new()),
        source_cache: RwLock::default(),
        arenas: GlobalArenas::default(),
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
        Command::Explain { ecode } => cli::commands::explain(&gcx, &ecode),
        Command::Internal { cmd } => cli::commands::internal(&gcx, &cmd),
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

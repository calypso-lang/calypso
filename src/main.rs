#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso/index.html")]
#![warn(clippy::pedantic)]

use std::panic;
use std::sync::Arc;

use clap::{load_yaml, App};
use once_cell::sync::OnceCell;
use tracing_subscriber::EnvFilter;

use calypso_base::session::BaseSession;
use calypso_base::ui::{self, atty::Stream};

mod buildinfo;
mod commands;

use buildinfo::BUILD_INFO;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

static DEFAULT_HOOK: OnceCell<Box<dyn Fn(&panic::PanicInfo<'_>) + Sync + Send + 'static>> =
    OnceCell::new();

const BUG_REPORT_URL: &str = "https://github.com/calypso-lang/calypso/issues/new\
    ?assignees=&labels=C-bug&template=bug-report.md&title=bug%3A+";

fn init_panic_hook(sess: Arc<BaseSession>) {
    // This is dumb but borrowck really wants me to do it this way. Luckily the
    // remaining useless `Arc`s will just be dropped, and this is just init
    // code.
    let sess = Arc::clone(&sess);
    DEFAULT_HOOK.get_or_init(|| {
        let sess = Arc::clone(&sess);
        let hook = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            let sess = Arc::clone(&sess);
            report_ice(&*sess, info, BUG_REPORT_URL)
        }));
        hook
    });
}

fn report_ice(sess: &BaseSession, info: &panic::PanicInfo<'_>, bug_report_url: &str) {
    // Invoke the default handler, which prints the actual panic message and
    // optionally a backtrace
    DEFAULT_HOOK.get().unwrap()(info);

    // Separate the output with an empty line
    eprintln!();

    ui::error_to(
        &sess.stderr,
        None,
        "the compiler unexpectedly crashed. this is a bug.",
        None,
    )
    .unwrap();
    ui::note_to(
        &sess.stderr,
        "we would appreciate a bug report at",
        Some(bug_report_url),
    )
    .unwrap();
    ui::note_to(
        &sess.stderr,
        "build information",
        Some(&format!(
            "calypso {} ({}) running on {}",
            BUILD_INFO.version, BUILD_INFO.git_commit, BUILD_INFO.cargo_target_triple
        )),
    )
    .unwrap();
    ui::note_to(
        &sess.stderr,
        "for further information, run",
        Some("`calypso internal buildinfo`"),
    )
    .unwrap();
}

fn main() {
    let yaml = load_yaml!("data/cli-en.yml");
    let matches = App::from_yaml(yaml)
        .version(BUILD_INFO.version)
        .get_matches();

    let color_pref = matches.value_of("color").unwrap();
    let color_pref_stdout = ui::parse_color_pref(color_pref, Stream::Stdout);
    let color_pref_stderr = ui::parse_color_pref(color_pref, Stream::Stderr);
    let sess = Arc::new(BaseSession::new(color_pref_stdout, color_pref_stderr));

    init_panic_hook(Arc::clone(&sess));
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_env("CALYPSO_LOG"))
        .pretty()
        .init();

    match matches.subcommand() {
        ("internal", Some(matches)) => commands::internal(sess, matches),
        ("explain", Some(matches)) => commands::explain(sess, matches),
        _ => unreachable!(),
    }
}

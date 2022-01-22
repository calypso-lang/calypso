use std::sync::Arc;

use calypso_base::ui::termcolor::{Color, ColorSpec, WriteColor};
use calypso_common::gcx::GlobalCtxt;
use calypso_diagnostic::prelude::*;

use crate::buildinfo::BUILD_INFO;
use crate::cli::InternalCmd;
use crate::commands::internal::unpretty::unpretty;

pub mod unpretty;

#[allow(clippy::single_match)]
pub fn internal(gcx: &Arc<GlobalCtxt>, cmd: &InternalCmd) -> CalResult<()> {
    match cmd {
        InternalCmd::BuildInfo => buildinfo(gcx),
        InternalCmd::Panic => panic!("Intentional panic to test ICE handling, please ignore."),
        InternalCmd::Unpretty {
            format,
            input,
            repl,
        } => unpretty(gcx, *format, input.as_ref(), *repl),
    }
}

pub fn buildinfo(gcx: &Arc<GlobalCtxt>) -> CalResult<()> {
    let mut bi = BUILD_INFO;

    let mut emit = gcx.emit.write();
    let out = &mut emit.out;

    out.info("=:= Version =:=", None)?
        .newline()?
        .info("version", Some(bi.version))?
        .info("git branch", Some(bi.git_branch))?
        .info("git commit", Some(bi.git_commit))?
        .newline()?
        .info("=:= Build Env =:=", None)?
        .newline()?
        .info("features:", None)?;

    if bi.cargo_features.is_empty() {
        bi.cargo_features = "no cargo features enabled";
    }

    for feature in bi.cargo_features.split(',') {
        out.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Green))
                .set_bold(true)
                .set_intense(true),
        )?;
        out.print("  =>")?;
        out.reset()?;
        out.print(&format!(" {}", feature))?.newline()?;
    }

    out.info("profile", Some(bi.cargo_profile))?
        .info("target triple", Some(bi.cargo_target_triple))?
        .newline()?
        .info("=:= Rust =:=", None)?
        .newline()?
        .info("channel", Some(bi.rustc_channel))?
        .info("commit date", Some(bi.rustc_commit_date))?
        .info("commit hash", Some(bi.rustc_commit_hash))?
        .info("host triple", Some(bi.rustc_host_triple))?
        .info("llvm version", Some(bi.rustc_llvm_version))?
        .info("version", Some(bi.rustc_version))?
        .flush()?;

    Ok(())
}

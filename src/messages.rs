use std::fmt::Display;

use ansi_term::Color;

use calypso_diagnostic::prelude::*;

pub fn error<E: Display>(error: E) {
    eprintln!(
        "{}{}",
        Color::Red.bold().paint("error"),
        Color::White.bold().paint(format!(": {}", error))
    );
}

pub fn _error_category<C: Display, E: Display>(category: C, error: E) {
    println!(
        "{}{}",
        Color::Red.bold().paint(format!("error[{}]", category)),
        Color::White.bold().paint(format!(": {}", error))
    );
}

pub fn info<C: Display, I: Display>(category: C, info: I) {
    println!(
        "{}{}",
        Color::Cyan.bold().paint(format!("{}", category)),
        Color::White.bold().paint(format!(": {}", info))
    );
}

pub fn note<C: Display, N: Display>(category: C, note: N) {
    println!(
        "{}{}",
        Color::Green.bold().paint(format!("{}", category)),
        Color::White.bold().paint(format!(": {}", note))
    );
}

pub fn _warn<W: Display>(warning: W) {
    eprintln!(
        "{}{}",
        Color::Yellow.bold().paint("warning"),
        Color::White.bold().paint(format!(": {}", warning))
    );
}

pub fn error_chained(err: impl Into<CalError>) {
    let err = err.into();
    match err.kind() {
        CalErrorKind::Diagnostic(..) => println!("{}", &err),
        _ => error(&err),
    }
    for e in err.iter().skip(1) {
        note("caused by", e);
    }
    if let Some(backtrace) = err.backtrace() {
        info(
            "a backtrace is available",
            format_args!("\n{:?}", backtrace),
        );
    }
}

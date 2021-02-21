use std::fmt::Display;

use ansi_term::Color;

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

pub fn _info<C: Display, I: Display>(category: C, info: I) {
    println!(
        "{}{}",
        Color::Cyan.bold().paint(format!("{}", category)),
        Color::White.bold().paint(format!(": {}", info))
    );
}

pub fn _note<C: Display, N: Display>(category: C, note: N) {
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

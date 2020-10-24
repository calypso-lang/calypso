use std::fmt::Display;

use ansi_term::Color;

pub fn error<E: Display>(error: E) {
    eprintln!(
        "{}{}",
        Color::Red.bold().paint("error"),
        Color::White.bold().paint(format!(": {}", error))
    );
}

pub fn info<C: Display, I: Display>(category: C, info: I) {
    println!(
        "{}{}",
        Color::Green.bold().paint(format!("{}", category)),
        Color::White.bold().paint(format!(": {}", info))
    );
}

pub fn warn<W: Display>(warning: W) {
    eprintln!(
        "{}{}",
        Color::Yellow.bold().paint("warning"),
        Color::White.bold().paint(format!(": {}", warning))
    );
}

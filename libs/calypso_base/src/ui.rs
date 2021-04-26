use std::io::prelude::*;

use atty::Stream;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use calypso_error::CalResult;

pub use atty;
pub use termcolor;

/// Parse a color preference (`always`, `ansi`, `auto`, anything else: auto) to
/// a color preference. Uses [`atty`] internally to test if the output stream
/// is a TTY.
pub fn parse_color_pref(pref: &str, stream: Stream) -> ColorChoice {
    match pref {
        "always" => ColorChoice::Always,
        "ansi" => ColorChoice::AlwaysAnsi,
        "auto" => {
            if atty::is(stream) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }
        }
        _ => ColorChoice::Never,
    }
}

pub fn error(
    buffer: &mut Buffer,
    code: Option<&str>,
    short: &str,
    message: Option<&str>,
) -> CalResult<()> {
    buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::Red))
            .set_bold(true)
            .set_intense(true),
    )?;
    write!(buffer, "error")?;
    if let Some(code) = code {
        write!(buffer, "[{}]", code)?;
    }
    write!(buffer, ": ")?;
    buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_bold(true)
            .set_intense(true),
    )?;
    write!(buffer, "{}", short)?;
    if let Some(message) = message {
        write!(buffer, ": ")?;
        buffer.reset()?;
        write!(buffer, "{}", message)?;
    }
    writeln!(buffer)?;
    Ok(())
}

fn message_general(
    buffer: &mut Buffer,
    main: &'static str,
    color: Color,
    short: &str,
    message: Option<&str>,
) -> CalResult<()> {
    buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(color))
            .set_bold(true)
            .set_intense(true),
    )?;
    write!(buffer, "{}: ", main)?;
    buffer.set_color(
        ColorSpec::new()
            .set_fg(Some(Color::White))
            .set_bold(true)
            .set_intense(true),
    )?;
    write!(buffer, "{}", short)?;
    if let Some(message) = message {
        write!(buffer, ": ")?;
        buffer.reset()?;
        write!(buffer, "{}", message)?;
    }
    writeln!(buffer)?;
    Ok(())
}

pub fn info(buffer: &mut Buffer, short: &str, message: Option<&str>) -> CalResult<()> {
    message_general(buffer, "info", Color::Cyan, short, message)
}

pub fn note(buffer: &mut Buffer, short: &str, message: Option<&str>) -> CalResult<()> {
    message_general(buffer, "note", Color::Green, short, message)
}

pub fn warn(buffer: &mut Buffer, short: &str, message: Option<&str>) -> CalResult<()> {
    message_general(buffer, "warn", Color::Yellow, short, message)
}

pub fn error_to(
    writer: &BufferWriter,
    code: Option<&str>,
    short: &str,
    message: Option<&str>,
) -> CalResult<()> {
    let mut buffer = writer.buffer();
    error(&mut buffer, code, short, message)?;
    writer.print(&buffer)?;
    Ok(())
}

pub fn info_to(writer: &BufferWriter, short: &str, message: Option<&str>) -> CalResult<()> {
    let mut buffer = writer.buffer();
    info(&mut buffer, short, message)?;
    writer.print(&buffer)?;
    Ok(())
}

pub fn note_to(writer: &BufferWriter, short: &str, message: Option<&str>) -> CalResult<()> {
    let mut buffer = writer.buffer();
    note(&mut buffer, short, message)?;
    writer.print(&buffer)?;
    Ok(())
}

pub fn warn_to(writer: &BufferWriter, short: &str, message: Option<&str>) -> CalResult<()> {
    let mut buffer = writer.buffer();
    warn(&mut buffer, short, message)?;
    writer.print(&buffer)?;
    Ok(())
}

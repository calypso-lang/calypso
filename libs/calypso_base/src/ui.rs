use std::io::prelude::*;
use std::ops::{Deref, DerefMut};

use atty::Stream;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use calypso_error::CalResult;

pub use atty;
pub use termcolor;

/// Parse a color preference (`always`, `ansi`, `auto`, anything else: auto) to
/// a color preference. Uses [`atty`] internally to test if the output stream
/// is a TTY.
#[must_use]
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

/// A helper struct containing the emitters for stdout and stderr.
pub struct Emitters {
    /// Emits to stdout
    pub out: Emitter,
    /// Emits to stderr
    pub err: Emitter,
}

impl Emitters {
    /// Create a new instance of this structure with the given preferences for
    /// color output for each stream. These can be parsed from text using
    /// [`parse_color_pref`].
    #[must_use]
    pub fn new(out_colors: ColorChoice, err_colors: ColorChoice) -> Self {
        {
            Self {
                out: Emitter::new(BufferWriter::stdout(out_colors)),
                err: Emitter::new(BufferWriter::stdout(err_colors)),
            }
        }
    }
}

/// A structure handling emitting messages to the terminal. This [`Deref`]s and
/// [`DerefMut`]s to a [`termcolor::Buffer`], so you can use
/// [`termcolor::WriteColor`] and methods on `Buffer` to write custom text.
///
/// Note that text is never automatically flushed, so you must manually use
/// [`Emitter::flush`].
pub struct Emitter {
    writer: BufferWriter,
    buf: Buffer,
}

impl Emitter {
    /// Create a new emitter.
    pub fn new(writer: BufferWriter) -> Self {
        let buf = writer.buffer();
        Self { writer, buf }
    }

    /// Create a new buffer based on the color preferences of this emitter.
    pub fn buffer(&self) -> Buffer {
        self.writer.buffer()
    }

    /// Flush the emitter. This will clear the internal buffer.
    ///
    /// # Errors
    ///
    /// This function will error if the emitter could not print the contents of
    /// the internal buffer.
    pub fn flush(&mut self) -> CalResult<&mut Self> {
        self.writer.print(&self.buf)?;
        self.buf.clear();
        Ok(self)
    }

    /// Add a newline to the internal buffer.
    ///
    /// # Errors
    ///
    /// This function will error if the buffer could not be updated.
    pub fn newline(&mut self) -> CalResult<&mut Self> {
        writeln!(self.buf)?;
        Ok(self)
    }

    /// Add the string provided to the internal buffer verbatim. (Chainable)
    ///
    /// # Errors
    ///
    /// This function will error if the buffer could not be updated.
    pub fn print(&mut self, s: &str) -> CalResult<&mut Self> {
        write!(self.buf, "{}", s)?;
        Ok(self)
    }

    /// Emit an error. Note that this function **will** reset the existing
    /// color of the internal buffer. The emitted text will have a newline.
    ///
    /// # Forms
    ///
    /// Angle brackets (`<>`) indicate a string provided to the function.
    ///
    /// ## Without `code` or `message`
    ///
    /// ```text
    /// error: <short>
    /// ```
    ///
    /// ## With `code` but not `message`
    ///
    /// ```text
    /// error[<code>]: <short>
    /// ```
    ///
    /// ## With `message` but not `code`
    ///
    /// ```text
    /// error: <short>: <message>
    /// ```
    ///
    /// ## With `code` and `message`
    ///
    /// ```text
    /// error[<code>]: <short>: <message>
    /// ```
    ///
    /// # Color
    ///
    /// When color is enabled for the output provided, the segments will be
    /// colored as such:
    ///
    /// - `error[<code>]`: Red; bold, intense
    /// - `<short>`: White; bold, intense
    /// - `<message>`: Default color
    ///
    /// # Errors
    ///
    /// This function will error if at any point text could not be written to
    /// the internal buffer.
    pub fn error(
        &mut self,
        code: Option<&str>,
        short: &str,
        message: Option<&str>,
    ) -> CalResult<&mut Self> {
        self.buf.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Red))
                .set_bold(true)
                .set_intense(true),
        )?;
        write!(self.buf, "error")?;
        if let Some(code) = code {
            write!(self.buf, "[{}]", code)?;
        }
        write!(self.buf, ": ")?;
        self.buf.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::White))
                .set_bold(true)
                .set_intense(true),
        )?;
        write!(self.buf, "{}", short)?;
        if let Some(message) = message {
            write!(self.buf, ": ")?;
            self.buf.reset()?;
            write!(self.buf, "{}", message)?;
        }
        writeln!(self.buf)?;
        Ok(self)
    }

    fn message_general(
        &mut self,
        main: &'static str,
        color: Color,
        short: &str,
        message: Option<&str>,
    ) -> CalResult<&mut Self> {
        self.buf.set_color(
            ColorSpec::new()
                .set_fg(Some(color))
                .set_bold(true)
                .set_intense(true),
        )?;
        write!(self.buf, "{}: ", main)?;
        self.buf.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::White))
                .set_bold(true)
                .set_intense(true),
        )?;
        write!(self.buf, "{}", short)?;
        if let Some(message) = message {
            write!(self.buf, ": ")?;
            self.buf.reset()?;
            write!(self.buf, "{}", message)?;
        }
        writeln!(self.buf)?;
        Ok(self)
    }

    /// Emit an informational message. Note that this function **will** reset
    /// the existing color of the internal buffer. The emitted text will have a
    /// newline.
    ///
    /// # Forms
    ///
    /// Angle brackets (`<>`) indicate a string provided to the function.
    ///
    /// ## Without `message`
    ///
    /// ```text
    /// info: <short>
    /// ```
    ///
    /// ## With `message`
    ///
    /// ```text
    /// info: <short>: <message>
    /// ```
    ///
    /// # Color
    ///
    /// When color is enabled for the output provided, the segments will be
    /// colored as such:
    ///
    /// - `info`: Cyan; bold, intense
    /// - `<short>`: White; bold, intense
    /// - `<message>`: Default color
    ///
    /// # Errors
    ///
    /// This function will error if at any point text could not be written to
    /// the internal buffer.
    pub fn info(&mut self, short: &str, message: Option<&str>) -> CalResult<&mut Self> {
        self.message_general("info", Color::Cyan, short, message)
    }

    /// Emit a note. Note that this function **will** reset the existing color
    /// of the internal buffer. The emitted text will have a newline.
    ///
    /// # Forms
    ///
    /// Angle brackets (`<>`) indicate a string provided to the function.
    ///
    /// ## Without `message`
    ///
    /// ```text
    /// note: <short>
    /// ```
    ///
    /// ## With `message`
    ///
    /// ```text
    /// note: <short>: <message>
    /// ```
    ///
    /// # Color
    ///
    /// When color is enabled for the output provided, the segments will be
    /// colored as such:
    ///
    /// - `note`: Green; bold, intense
    /// - `<short>`: White; bold, intense
    /// - `<message>`: Default color
    ///
    /// # Errors
    ///
    /// This function will error if at any point text could not be written to
    /// the internal buffer.
    pub fn note(&mut self, short: &str, message: Option<&str>) -> CalResult<&mut Self> {
        self.message_general("note", Color::Green, short, message)
    }

    /// Emit a warning. Note that this function **will** reset the existing
    /// color of the internal buffer. The emitted text will have a newline.
    ///
    /// # Forms
    ///
    /// Angle brackets (`<>`) indicate a string provided to the function.
    ///
    /// ## Without `message`
    ///
    /// ```text
    /// warn: <short>
    /// ```
    ///
    /// ## With `message`
    ///
    /// ```text
    /// warn: <short>: <message>
    /// ```
    ///
    /// # Color
    ///
    /// When color is enabled for the output provided, the segments will be
    /// colored as such:
    ///
    /// - `warn`: Yellow; bold, intense
    /// - `<short>`: White; bold, intense
    /// - `<message>`: Default color
    ///
    /// # Errors
    ///
    /// This function will error if at any point text could not be written to
    /// the internal buffer.
    pub fn warn(&mut self, short: &str, message: Option<&str>) -> CalResult<&mut Self> {
        self.message_general("warn", Color::Yellow, short, message)
    }

    /// Emit a `Buffer` now.
    ///
    /// Since we can't merge buffers (see
    /// [termcolor#45](https://github.com/BurntSushi/termcolor/issues/45)),
    /// this function directly prints the buffer instead of extending the
    /// internal buffer with this one.
    ///
    /// # Errors
    ///
    /// This function will error
    pub fn emit(&mut self, buf: &Buffer) -> CalResult<&mut Self> {
        self.writer.print(&buf)?;
        Ok(self)
    }
}

impl Deref for Emitter {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl DerefMut for Emitter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buf
    }
}

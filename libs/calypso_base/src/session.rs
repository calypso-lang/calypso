use termcolor::{BufferWriter, ColorChoice};

pub struct BaseSession {
    pub stdout: BufferWriter,
    pub stderr: BufferWriter,
}

impl BaseSession {
    pub fn new(stdout: ColorChoice, stderr: ColorChoice) -> Self {
        Self {
            stdout: BufferWriter::stdout(stdout),
            stderr: BufferWriter::stderr(stderr),
        }
    }
}

use termcolor::{BufferWriter, ColorChoice};

use crate::sourcemgr::SourceMgr;

#[allow(clippy::module_name_repetitions)]
pub struct BaseSession {
    pub stdout: BufferWriter,
    pub stderr: BufferWriter,
    pub sourcemgr: SourceMgr,
}

impl BaseSession {
    #[must_use]
    pub fn new(stdout: ColorChoice, stderr: ColorChoice, sourcemgr: SourceMgr) -> Self {
        Self {
            stdout: BufferWriter::stdout(stdout),
            stderr: BufferWriter::stderr(stderr),
            sourcemgr,
        }
    }
}

use calypso_base::span::Span;

use super::types::DiagnosticId;
use std::fmt;

#[derive(Clone)]
pub struct Diagnostic {
    span: Span,
    line: usize,
    column: usize,
    line_str: String,
    source_name: String,
    reason: String,
    eid: DiagnosticId,
}

impl Diagnostic {
    pub fn new(span: Span, buffer: &[char], source_name: String, reason: String, eid: u16) -> Self {
        if !span.is_valid_for(&buffer) {
            panic!("Span is not valid for diagnostic");
        }
        let mut line = 1;
        let mut col = 1;
        for ch in buffer.iter().take(span.start()) {
            if *ch == '\n' {
                line += 1;
            } else {
                col += 1;
            }
        }
        let source_buf_string = buffer.to_vec().iter().collect::<String>();
        // FIXME: edge case: truncate line e.g. ...[really long line]<error>[really long line]...
        let source_line = source_buf_string.split('\n').nth(line - 1).unwrap();

        Self {
            span,
            line,
            column: col,
            line_str: source_line.to_string(),
            source_name,
            reason,
            eid: DiagnosticId::from(eid),
        }
    }

    pub fn eid(&self) -> &DiagnosticId {
        &self.eid
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn source_name(&self) -> &str {
        &self.source_name
    }
}

impl fmt::Debug for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Diagnostic")
            .field("span", &self.span)
            .field("reason", &self.reason)
            .finish()
    }
}

/*
error: reason
  --> file:line:col
     |
line | some.error
     |      ^^^^^ reason

*/
impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Display color if alt flag set
        let diag_reason = self.eid.get_reason().unwrap();
        writeln!(f, "error[E{:04}]: {}", self.eid.0, diag_reason)?;

        writeln!(f, " --> {}:{}:{}", self.source_name, self.line, self.column)?;
        let line_no = self.line.to_string();
        let padding = " ".repeat(line_no.chars().count());

        let carets = "^".repeat(self.span.length());
        let pre_caret_padding = " ".repeat(self.column - 1);
        writeln!(f, "{} |", padding)?;
        writeln!(f, "{} | {}", line_no, self.line_str)?;
        writeln!(
            f,
            "{} | {}{} {}",
            padding, pre_caret_padding, carets, self.reason
        )
    }
}

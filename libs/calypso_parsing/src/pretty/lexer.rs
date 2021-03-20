use super::Printer;
use crate::lexer::{Token, TokenType};

use calypso_diagnostic::prelude::*;
use calypso_diagnostic::reporting::files::Files;

impl Printer<'_> {
    /// Print a token
    ///
    /// # Errors
    ///
    /// This function errors when it can't get the line/col location of a token from the byte span.
    pub fn print_token(&mut self, tok: &Token<'_>) -> CalResult<String> {
        let value = tok.value();
        let span = tok.span();
        let lo = span.lo();
        let hi = span.hi();
        let lo_loc = self
            .fmgr
            .location(self.file_id, lo)
            .map_err(DiagnosticError::from)?;
        let hi_loc = self
            .fmgr
            .location(self.file_id, hi)
            .map_err(DiagnosticError::from)?;
        Ok(format!(
            "text: {} @ {}..{} (a.k.a. {}:{}..{}:{}), type: {:?}",
            match value.0 {
                TokenType::Ws => "omitted".to_string(),
                TokenType::Eof => "inapplicable".to_string(),
                _ => format!("`{}`", value.1),
            },
            lo,
            hi,
            lo_loc.line_number,
            lo_loc.column_number,
            hi_loc.line_number,
            hi_loc.column_number,
            value.0,
        ))
    }
}

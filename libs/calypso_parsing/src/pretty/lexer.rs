use super::Printer;
use crate::lexer::{Lexeme, Token};

use calypso_diagnostic::prelude::*;
use calypso_diagnostic::reporting::files::Files;

impl Printer {
    /// Print a token
    ///
    /// # Errors
    ///
    /// This function errors when it can't get the line/col location of a token from the byte span.
    pub fn print_token(&mut self, tok: &Lexeme<'_>) -> CalResult<String> {
        let value = tok.value();
        let span = tok.span();
        let lo = span.lo();
        let hi = span.hi();

        let sourcemgr = self.gcx.sourcemgr.read();
        let lo_loc = sourcemgr
            .location(self.file_id, lo)
            .map_err(DiagnosticError::from)?;
        let hi_loc = sourcemgr
            .location(self.file_id, hi)
            .map_err(DiagnosticError::from)?;
        drop(sourcemgr);

        Ok(format!(
            "text: {} @ {}..{} (a.k.a. {}:{}..{}:{}), type: {:?}",
            if let Token::Nl(_) = value.0 {
                "omitted".to_string()
            } else {
                format!("`{}`", value.1)
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

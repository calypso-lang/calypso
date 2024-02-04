use crate::{error::CalResult, parse::lexer::Lexeme, symbol::Symbol};

use super::Printer;

impl<'gcx> Printer<'gcx> {
    /// Print a token
    ///
    /// # Errors
    ///
    /// This function errors when it can't get the line/col location of a token from the byte span.
    pub fn print_token(&mut self, file: Symbol, tok: &Lexeme<'_>) -> CalResult<String> {
        let value = tok.value();
        let span = tok.span();
        let lo = span.lo();
        let hi = span.hi();

        let cache = self.gcx.source_cache.borrow();
        let (_, lo_line, lo_col) = cache
            .get(file)
            .expect("failed to fetch file")
            .get_offset_line(lo as usize)
            .expect("failed to fetch offset");
        let (_, hi_line, hi_col) = cache
            .get(file)
            .expect("failed to fetch file")
            .get_offset_line(hi as usize)
            .expect("failed to fetch offset");

        Ok(format!(
            "text: `{}` @ {}..{} (a.k.a. {}:{}..{}:{}), type: {:?}",
            value.1,
            lo,
            hi,
            lo_line + 1,
            lo_col + 1,
            hi_line + 1,
            hi_col + 1,
            value.0,
        ))
    }
}

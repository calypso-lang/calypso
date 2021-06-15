use super::helpers::{is_ident_continue, is_ident_end};
use super::{Lexer, Token, TokenType};

use calypso_base::streams::Stream;
use calypso_base::symbol::Symbol;
use calypso_diagnostic::diagnostic::{EnsembleBuilder, LabelStyle};
use calypso_diagnostic::prelude::*;

impl<'lex> Lexer<'lex> {
    pub(super) fn handle_identifier(&mut self) -> Token<'lex> {
        // `_` is not an ident on its own, but all other [A-Za-z]{1} idents are.
        if self.prev().unwrap() == &'_' && self.peek_cond(is_ident_continue) != Some(true) {
            return self.new_token(TokenType::Under);
        }

        // Gorge while the character is a valid identifier character (and not an ident_end character).
        self.gorge_while(|sp, _| is_ident_continue(sp) && !is_ident_end(sp));

        // // Allow `abc!`, `abc?`, and `abc!?` but not `abc?!`
        // if self.peek_eq(&'!') == Some(true) {
        //     self.next();
        // }
        // if self.peek_eq(&'?') == Some(true) {
        //     self.next();
        // }

        let span = self.new_span();
        let sliced = self.slice(span);

        if sliced.len() > 255 {
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0034").short(err!(E0034)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            span.shrink_to_lo(),
                        )
                    })
                    .build(),
            );
        }

        let ident = Symbol::intern(sliced);
        self.new_token(if ident.is_keyword() {
            TokenType::Keyword(ident)
        } else {
            TokenType::Ident(ident)
        })
    }
}

use radix_trie::Trie;

use super::helpers::{is_ident_continue, is_ident_end};
use super::{Keyword, Lexer, Token, TokenType};

use calypso_base::init_trie;
use calypso_base::streams::Stream;
use calypso_diagnostic::prelude::*;

init_trie!(pub KEYWORD_TRIE: Keyword => {
    "false" => False,
    "null"  => Null,
    "true"  => True
});

impl<'lex> Lexer<'lex> {
    pub(super) fn handle_identifier(&mut self) -> CalResult<Token<'lex>> {
        let mut token_type = TokenType::Ident;

        // `_` is not an ident on its own, but all other [A-Za-z]{1} idents are.
        if self.prev().unwrap() == &'_' && self.peek_cond(is_ident_continue) != Some(true) {
            return Ok(self.new_token(TokenType::Under));
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
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0034;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, span.shrink_to_lo());
                        "this identifier was too long"
                ]
            });
        }

        let keyword = KEYWORD_TRIE.get(sliced);

        if let Some(&keyword) = keyword {
            token_type = TokenType::Keyword(keyword);
        }

        Ok(self.new_token(token_type))
    }
}

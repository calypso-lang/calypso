use radix_trie::Trie;

use super::helpers::{is_ident_continue, is_ident_end};
use super::{Keyword, Lexer, Token, TokenType};

use calypso_base::init_trie;
use calypso_base::streams::Stream;
use calypso_diagnostic::prelude::*;

init_trie!(pub KEYWORD_TRIE: Keyword => {
    "is"     => Is,
    "isa"    => Isa,
    "bool"   => BoolTy,
    "sint"   => SintTy,
    "uint"   => UintTy,
    "float"  => FloatTy,
    "string" => StringTy,
    "char"   => CharTy,
    "tuple"  => TupleTy,
    "array"  => ArrayTy,
    "false"  => False,
    "true"   => True,
    "if"     => If,
    "else"   => Else,
    "for"    => For,
    "in"     => In,
    "loop"   => Loop,
    "while"  => While,
    "case"   => Case,
    "cond"   => Cond,
    "ret"    => Ret,
    "break"  => Break,
    "fn"     => Fn,
    "native" => Extern,
    "mod"    => Mod,
    "use"    => Use,
    "import" => Import,
    "pub"    => Pub,
    "let"    => Let,
    "mut"    => Mut,
    "undef"  => Undef,
    "null"   => Null,
    "del"    => Del,
    "as"     => As,
    "panic"  => Panic
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

        // Allow `abc!`, `abc?`, and `abc!?` but not `abc?!`
        if self.peek_eq(&'!') == Some(true) {
            self.next();
        }
        if self.peek_eq(&'?') == Some(true) {
            self.next();
        }

        let keyword = KEYWORD_TRIE.get(&self.slice(self.new_span()).to_string());

        if let Some(&keyword) = keyword {
            token_type = TokenType::Keyword(keyword);
        }

        Ok(self.new_token(token_type))
    }
}

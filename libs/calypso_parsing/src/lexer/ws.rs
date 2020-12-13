use super::helpers::is_whitespace;
use super::{Lexer, Token, TokenType};

use calypso_base::streams::Stream;
use calypso_diagnostic::prelude::*;

impl<'lex> Lexer<'lex> {
    pub(super) fn handle_whitespace(&mut self) -> CalResult<Option<Token<'lex>>> {
        self.current_to_start();
        self.handle_dangling_comment_ends()?;
        while !self.is_at_end()
            && (self.handle_comment()
                || self.handle_multiline_comment()?
                || self.next_if(is_whitespace).is_some())
        {
            self.handle_dangling_comment_ends()?;
        }
        if self.new_span().is_empty() {
            Ok(None)
        } else {
            Ok(Some(self.new_token(TokenType::Ws)))
        }
    }

    pub(super) fn handle_comment(&mut self) -> bool {
        // xx -> 11 -> 1
        // x/ -> 10 -> 1
        // /x -> 01 -> 1
        // // -> 00 -> 0
        if self.peek_eq(&'/') != Some(true) || self.peek2_eq(&'/') != Some(true) {
            return false;
        }
        // A comment goes until the end of the line,
        // so gorge all the characters until we get to the newline
        // (or the end, when it automatically stops gorging).
        self.gorge_while(|spanned, _| spanned != &'\n');
        true
    }

    pub(super) fn handle_multiline_comment(&mut self) -> CalResult<bool> {
        // xx -> 11 -> 1
        // x* -> 10 -> 1
        // /x -> 01 -> 1
        // /* -> 00 -> 0
        if self.peek_eq(&'/') != Some(true) || self.peek2_eq(&'*') != Some(true) {
            return Ok(false);
        }
        let start = self.start;
        self.current_to_start();
        self.next();
        self.next();
        let mut stack = vec![self.new_span()];

        loop {
            let span = self.peek();
            let ch = span.map(|sp| sp.value_owned());

            if stack.is_empty() {
                break;
            }

            if ch == Some('/') && self.peek2_eq(&'*') == Some(true) {
                self.current_to_start();
                self.next();
                self.next();
                stack.push(self.new_span());
            } else if ch == Some('*') && self.peek2_eq(&'/') == Some(true) {
                self.current_to_start();
                self.next();
                self.next();
                stack.pop();
            } else {
                self.next();
            }

            if self.is_at_end() && !stack.is_empty() {
                // There's no way to tell whether stuff after a /* was intended to be a comment
                // or code, so we make this a fatal error.
                gen_error!(Err(self => {
                    E0002;
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, stack.pop().unwrap());
                            "this multi-line comment's beginning has no corresponding end"
                    ]
                }) as ())?
            }
        }

        self.set_start(start);
        Ok(true)
    }

    pub(super) fn handle_dangling_comment_ends(&mut self) -> CalResult<()> {
        if self.peek_eq(&'*') == Some(true) && self.peek2_eq(&'/') == Some(true) {
            self.current_to_start();
            self.next();
            self.next();
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0001;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "this multi-line comment's end has no corresponding beginning"
                ]
            });
        }
        Ok(())
    }
}

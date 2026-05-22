use std::fmt;

use crate::{
    ctxt::GlobalCtxt,
    symbol::{self, Ident, Symbol, prim_ty::PrimitiveTy},
    syntax::ast::TyKind,
};

use super::{
    ast::Ty,
    error::{SyntaxError, SyntaxErrorKind},
    lexer::SpanTok,
    span::Span,
    token::Token,
};

pub struct Parser<'gcx, I: Iterator<Item = SpanTok>> {
    tokens: I,
    gcx: &'gcx GlobalCtxt,
    file: Symbol,
    peek0: SpanTok,
    peek1: Option<SpanTok>,
}

impl<I: Iterator<Item = SpanTok>> fmt::Debug for Parser<'_, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parser")
            .field("file", &self.file)
            .field("peek0", &self.peek0)
            .field("peek1", &self.peek1)
            .finish_non_exhaustive()
    }
}

impl<'gcx, I: Iterator<Item = SpanTok>> Parser<'gcx, I> {
    pub fn new(gcx: &'gcx GlobalCtxt, file: Symbol, tokens: I) -> Self {
        let mut this = Self {
            tokens,
            gcx,
            file,
            peek0: (Span::new_dummy(), Token::Eof),
            peek1: None,
        };
        this.peek0 = this.tokens.next().expect("token stream must end with eof");
        this.peek1 = this.tokens.next();
        this
    }

    fn report_error(&self, e: SyntaxError) {
        let report = e.into_report(self.gcx);
        let mut diag = self.gcx.diag.borrow_mut();
        diag.report_syncd(report);
    }

    #[must_use]
    fn advance(&mut self) -> SpanTok {
        let res = self.peek0;
        self.peek0 = self.peek1.expect("advance past end-of-file");
        self.peek1 = self.tokens.next();

        res
    }

    fn peek(&self) -> SpanTok {
        self.peek0
    }

    fn peek1(&self) -> Option<SpanTok> {
        self.peek1
    }

    fn expect(&mut self, expected: Token) -> Result<Span, Span> {
        let (span, found) = self.advance();
        if found == expected {
            Ok(span)
        } else {
            self.error_expected_found(span, expected.description(), found.description());
            Err(span)
        }
    }

    fn expect_ident(&mut self) -> Result<Ident, Span> {
        let (span, found) = self.advance();
        if let Token::Ident(symbol) = found {
            Ok(Ident { symbol, span })
        } else {
            self.error_expected_found(span, "identifier", found.description());
            Err(span)
        }
    }

    fn error_expected_found(&self, span: Span, expected: &'static str, found: &'static str) {
        self.report_error(SyntaxError {
            kind: SyntaxErrorKind::UnexpectedToken { expected, found },
            location: span,
        });
    }

    pub fn parse_ty_top(&mut self) -> Ty {
        self.parse_ty()
            .unwrap_or_else(|span| Ty::new(self.gcx, TyKind::Error, span))
    }

    fn parse_ty(&mut self) -> Result<Ty, Span> {
        match self.advance() {
            (span, Token::Eof) => {
                self.unexpected_eof(span);
                Err(span)
            }
            (span, Token::Ident(sym)) if symbol::prim_ty::is(sym) => Ok(Ty::new(
                self.gcx,
                TyKind::Primitive(PrimitiveTy::try_from(sym).unwrap()),
                span,
            )),
            (start, Token::Ident(sym)) if sym == *symbol::misc_ty::ARRAY => {
                self.expect(Token::LeftSquare).map_err(|sp| start.to(sp))?;
                let ty = self.parse_ty().map_err(|sp| start.to(sp))?;
                let end = self.expect(Token::RightSquare).map_err(|sp| start.to(sp))?;
                Ok(Ty::new(self.gcx, TyKind::Array(ty), start.to(end)))
            }
            (span, tok) => todo!("{span:?} {tok:?}"),
        }
    }

    pub fn unexpected_eof(&mut self, span: Span) {
        self.report_error(SyntaxError {
            kind: SyntaxErrorKind::UnexpectedEof,
            location: span,
        });
    }
}

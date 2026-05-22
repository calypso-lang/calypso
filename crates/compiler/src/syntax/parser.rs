use std::fmt;

use crate::{
    ctxt::GlobalCtxt,
    symbol::{self, Ident, Symbol, kw::Keyword, prim_ty::PrimitiveTy},
    syntax::ast::{TyKind, TypeDefnKind, VariantKind},
};

use super::{
    ast::{BinOpKind, Expr, ExprKind, Item, ItemKind, Modality, Ty, UnOpKind},
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
    suppress_error: bool,
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
            suppress_error: false,
        };
        this.peek0 = this.tokens.next().expect("token stream must end with eof");
        this.peek1 = this.tokens.next();
        this.suppress_error =
            this.peek0.1 == Token::Error || matches!(this.peek1, Some((_, Token::Error)));
        this
    }

    fn report_error(&self, e: SyntaxError) {
        // We suppress errors when the lexer gives us an error, as
        // lexer errors are probably fatal. Plus this parser wasn't
        // really designed with lexer errors in mind.
        // TODO: make not all lexer errors fatal
        if !self.suppress_error {
            let report = e.into_report(self.gcx);
            let mut diag = self.gcx.diag.borrow_mut();
            diag.report_syncd(report);
        }
    }

    #[must_use]
    fn advance(&mut self) -> SpanTok {
        let res = self.peek0;
        // lexer gives us a token stream followed by infinite eof's
        self.peek0 = self.peek1.unwrap();
        self.peek1 = self.tokens.next();

        if res.1 == Token::Error {
            self.suppress_error = true;
        }

        res
    }

    fn peek(&self) -> SpanTok {
        self.peek0
    }

    #[allow(unused)]
    fn peek1(&self) -> Option<SpanTok> {
        self.peek1
    }

    fn expect_matching(
        &mut self,
        expected: Token,
        matching: Token,
        at: Span,
    ) -> Result<Span, Span> {
        let (span, found) = self.peek();
        if found == expected {
            let _ = self.advance();
            Ok(span)
        } else {
            self.report_error(SyntaxError {
                kind: SyntaxErrorKind::ExpectedMatching {
                    expected: expected.description(),
                    to_match: matching.description(),
                    at,
                    found: found.description(),
                },
                location: span,
            });
            Err(span)
        }
    }

    fn expect(&mut self, expected: Token) -> Result<Span, Span> {
        let (span, found) = self.peek();
        if found == expected {
            let _ = self.advance();
            Ok(span)
        } else {
            self.error_expected_found(span, expected.description(), found.description());
            Err(span)
        }
    }

    fn expect_ident(&mut self) -> Result<Ident, Span> {
        let (span, found) = self.peek();
        if let Token::Ident(symbol) = found {
            let _ = self.advance();
            Ok(Ident { symbol, span })
        } else {
            self.error_expected_found(span, "identifier", found.description());
            Err(span)
        }
    }

    fn error_expected_found(&self, span: Span, expected: &'static str, found: &'static str) {
        self.report_error(SyntaxError {
            kind: SyntaxErrorKind::Unexpected { expected, found },
            location: span,
        });
    }

    fn maybe_skip_nls(&mut self, allow_nl: bool) {
        if allow_nl {
            self.skip_nls();
        }
    }

    fn skip_nls(&mut self) {
        while let (_, Token::Nl) = self.peek() {
            let _ = self.advance();
        }
    }

    #[allow(clippy::too_many_lines)]
    fn parse_ty(&mut self, allow_nl: bool) -> Result<Ty, Span> {
        self.maybe_skip_nls(allow_nl);
        match self.peek() {
            (span, Token::Ident(sym)) if symbol::prim_ty::is(sym) => {
                let _ = self.advance();
                Ok(Ty::new(
                    self.gcx,
                    TyKind::Primitive(PrimitiveTy::try_from(sym).unwrap()),
                    span,
                ))
            }
            (start, Token::Ident(sym)) if sym == *symbol::misc_ty::ARRAY => {
                let _ = self.advance();
                let lsquare = self.expect(Token::LeftSquare).map_err(|sp| start.to(sp))?;
                self.skip_nls();
                let ty = self.parse_ty(true).or_ty_error(self);
                self.skip_nls();
                let end = self
                    .expect_matching(Token::RightSquare, Token::LeftSquare, lsquare)
                    .map_err(|sp| start.to(sp))?;
                Ok(Ty::new(self.gcx, TyKind::Array(ty), start.to(end)))
            }
            (start, Token::Ident(sym)) if sym == *symbol::misc_ty::OPTION => {
                let _ = self.advance();
                let lsquare = self.expect(Token::LeftSquare).map_err(|sp| start.to(sp))?;
                self.skip_nls();
                let ty = self.parse_ty(true).or_ty_error(self);
                self.skip_nls();
                let end = self
                    .expect_matching(Token::RightSquare, Token::LeftSquare, lsquare)
                    .map_err(|sp| start.to(sp))?;
                Ok(Ty::new(self.gcx, TyKind::Option(ty), start.to(end)))
            }
            (start, Token::LeftParen) => {
                let _ = self.advance();
                let end = self
                    .expect_matching(Token::RightParen, Token::LeftParen, start)
                    .map_err(|sp| start.to(sp))?;
                Ok(Ty::new(self.gcx, TyKind::Unit, start.to(end)))
            }
            (start, Token::Keyword(Keyword::Unique)) => {
                let _ = self.advance();
                let ty = self.parse_ty(false).or_ty_error(self);
                Ok(Ty::new(
                    self.gcx,
                    TyKind::Modal(Modality::Unique, ty),
                    start.to(ty.get(self.gcx).span),
                ))
            }
            (start, Token::Keyword(Keyword::Ref)) => {
                let _ = self.advance();
                let mode = match self.peek() {
                    (_, Token::Keyword(Keyword::Mut)) => {
                        let _ = self.advance();
                        Modality::RefMut
                    }
                    (_, Token::Keyword(Keyword::Imm)) => {
                        let _ = self.advance();
                        Modality::RefImm
                    }
                    _ => Modality::Ref,
                };
                let ty = self.parse_ty(false).or_ty_error(self);
                Ok(Ty::new(
                    self.gcx,
                    TyKind::Modal(mode, ty),
                    start.to(ty.get(self.gcx).span),
                ))
            }
            (start, Token::Keyword(Keyword::Shared)) => {
                let _ = self.advance();
                let mode = match self.peek() {
                    (_, Token::Keyword(Keyword::Mut)) => {
                        let _ = self.advance();
                        Modality::SharedMut
                    }
                    (_, Token::Keyword(Keyword::Imm)) => {
                        let _ = self.advance();
                        Modality::SharedImm
                    }
                    (span, found) => {
                        self.error_expected_found(span, "`mut` or `imm`", found.description());
                        Modality::SharedMut
                    }
                };
                let ty = self.parse_ty(false).or_ty_error(self);
                Ok(Ty::new(
                    self.gcx,
                    TyKind::Modal(mode, ty),
                    start.to(ty.get(self.gcx).span),
                ))
            }
            (span, Token::Ident(symbol)) => {
                let _ = self.advance();
                Ok(Ty::new(
                    self.gcx,
                    TyKind::Ident(Ident { symbol, span }),
                    span,
                ))
            }
            (span, found) => {
                self.error_expected_found(span, "type", found.description());
                Err(span)
            }
        }
    }

    fn try_consume(&mut self, token: Token) -> Option<Span> {
        if self.peek().1 == token {
            let (span, _) = self.advance();
            Some(span)
        } else {
            None
        }
    }

    fn try_parse_ty_annotation(&mut self) -> Option<(Span, Ty)> {
        if let Some(start) = self.try_consume(Token::Colon) {
            let ty = self.parse_ty(true).or_ty_error(self);
            let end = ty.get(self.gcx).span;
            Some((start.to(end), ty))
        } else {
            None
        }
    }

    fn parse_stmt(&mut self) -> Result<Expr, Span> {
        match self.peek() {
            (start, Token::Keyword(Keyword::Let)) => {
                let _ = self.advance();
                self.skip_nls();
                let is_mut = self.try_consume(Token::Keyword(Keyword::Mut)).is_some();
                self.skip_nls();
                let ident = self.expect_ident().map_err(|sp| start.to(sp))?;
                self.skip_nls();
                let ty = self.try_parse_ty_annotation().map(|(_, ty)| ty);
                self.skip_nls();
                self.expect(Token::Eq).map_err(|sp| start.to(sp))?;
                self.skip_nls();
                let val = self.parse_expr(false).or_expr_error(self);

                let end = val.get(self.gcx).span;
                Ok(Expr::new(
                    self.gcx,
                    ExprKind::Let {
                        name: ident,
                        is_mut,
                        ty,
                        val,
                    },
                    start.to(end),
                ))
            }
            // We pass through as-is without returning an
            // `ExprKind::Error` because we don't wrap expr
            // statements.
            _ => self.parse_expr(false),
        }
    }

    fn span(&self) -> Span {
        self.peek0.0
    }

    /// Does not consume end-delimiting tokens. However, you can
    /// safely expect (without checking) that the closing delimiter
    /// exists.
    ///
    /// `expected_separator` should be of the form "`;`, newline, or <delimiter>"
    ///
    /// `expected_delimiter` should be of the form "<delimiter>"
    fn parse_stmts_block(
        &mut self,
        ending: impl Fn(Token) -> bool,
        expected_separator: &'static str,
        expected_delimiter: &'static str,
        start_delimiter: &'static str,
        start_delimiter_span: Span,
    ) -> (Span, im::Vector<Expr>) {
        self.parse_delimited(
            |this| this.parse_stmt().or_expr_error(this),
            ending,
            |tok| matches!(tok, Token::Semi | Token::Nl),
            |this, expr| expr.get(this.gcx).span,
            |_this, span| Expr::new(self.gcx, ExprKind::Error, span),
            expected_separator,
            expected_delimiter,
            Some((start_delimiter, start_delimiter_span)),
            true,
        )
    }

    fn parse_exprs_list(
        &mut self,
        ending: impl Fn(Token) -> bool,
        expected_separator: &'static str,
        expected_delimiter: &'static str,
        start_delimiter: &'static str,
        start_delimiter_span: Span,
    ) -> (Span, im::Vector<Expr>) {
        self.parse_delimited(
            |this| {
                this.skip_nls();
                let expr = this.parse_expr(true).or_expr_error(this);
                this.skip_nls();
                expr
            },
            ending,
            |tok| matches!(tok, Token::Comma),
            |this, expr| expr.get(this.gcx).span,
            |_this, span| Expr::new(self.gcx, ExprKind::Error, span),
            expected_separator,
            expected_delimiter,
            Some((start_delimiter, start_delimiter_span)),
            true,
        )
    }

    fn skip_until(&mut self, f: impl Fn(Token) -> bool) {
        while f(self.peek().1) {
            let _ = self.advance();
        }
    }

    fn skip_until2(&mut self, f: impl Fn(Token, Option<Token>) -> Option<Skip>) {
        while let Some(skip) = f(self.peek().1, self.peek1().map(|(_, x)| x)) {
            self.with_skip(skip);
        }
    }

    /// Does not consume end-delimiting tokens. However, you can
    /// safely expect (without checking) that the closing delimiter
    /// exists.
    ///
    /// `expected_separator` should be of the form "`,` or <delimiter>"
    ///
    /// `expected_delimiter` should be of the form "<delimiter>"
    #[allow(clippy::too_many_arguments)]
    fn parse_delimited<T: Clone>(
        &mut self,
        parse: impl Fn(&mut Self) -> T,
        ending: impl Fn(Token) -> bool,
        separator: impl Fn(Token) -> bool,
        get_span: impl Fn(&Self, &T) -> Span,
        make_error: impl Fn(&Self, Span) -> T,
        expected_separator: &'static str,
        expected_delimiter: &'static str,
        start_delimiter: Option<(&'static str, Span)>,
        allow_nl_at_start_and_end: bool,
    ) -> (Span, im::Vector<T>) {
        let start = self.span();
        let mut end = self.span();
        let mut parses = im::Vector::new();
        loop {
            self.maybe_skip_nls(allow_nl_at_start_and_end);
            if let Some((start_delimiter, start_delimiter_span)) = start_delimiter
                && matches!(self.peek().1, Token::Eof)
            {
                self.report_error(SyntaxError {
                    kind: SyntaxErrorKind::ExpectedMatching {
                        expected: expected_delimiter,
                        to_match: start_delimiter,
                        at: start_delimiter_span,
                        found: Token::Eof.description(),
                    },
                    location: self.peek().0,
                });
                break;
            } else if ending(self.peek().1) {
                break;
            }
            let parse = parse(self);
            end = get_span(self, &parse);
            parses.push_back(parse);
            match self.peek() {
                (_, tok) if separator(tok) => {
                    let _ = self.advance();
                }
                (_, tok) if ending(tok) => {
                    break;
                }
                (_, Token::Eof) => {
                    // Let the check at the top handle it.
                }
                (span, found) => {
                    let _ = self.advance();
                    self.error_expected_found(span, expected_separator, found.description());
                    parses.push_back(make_error(self, span));
                    self.skip_until(|tok| separator(tok) || ending(tok));
                }
            }
        }
        (start.to(end), parses)
    }

    /// Does not consume end-delimiting tokens. However, you can
    /// safely expect (without checking) that the closing delimiter
    /// exists.
    ///
    /// `expected_separator` should be of the form "`,` or <delimiter>"
    ///
    /// `expected_delimiter` should be of the form "<delimiter>"
    #[allow(clippy::too_many_arguments)]
    fn parse_delimited2<T: Clone>(
        &mut self,
        parse: impl Fn(&mut Self) -> T,
        ending: impl Fn(Token, Option<Token>) -> Option<Skip>,
        separator: impl Fn(Token, Option<Token>) -> Option<Skip>,
        get_span: impl Fn(&Self, &T) -> Span,
        make_error: impl Fn(&Self, Span) -> T,
        expected_separator: &'static str,
        expected_delimiter: &'static str,
        start_delimiter: Option<(&'static str, Span)>,
        allow_nl_at_start_and_end: bool,
    ) -> (Skip, Span, im::Vector<T>) {
        let start = self.span();
        let mut end = self.span();
        let mut parses = im::Vector::new();
        let skip;
        loop {
            self.maybe_skip_nls(allow_nl_at_start_and_end);
            if let Some(s) = ending(self.peek().1, self.peek1().map(|(_, x)| x)) {
                skip = s;
                break;
            } else if let Some((start_delimiter, start_delimiter_span)) = start_delimiter
                && matches!(self.peek().1, Token::Eof)
            {
                self.report_error(SyntaxError {
                    kind: SyntaxErrorKind::ExpectedMatching {
                        expected: expected_delimiter,
                        to_match: start_delimiter,
                        at: start_delimiter_span,
                        found: Token::Eof.description(),
                    },
                    location: self.peek().0,
                });
                skip = Skip::One;
                break;
            }
            let parse = parse(self);
            end = get_span(self, &parse);
            parses.push_back(parse);
            match (
                self.peek(),
                separator(self.peek().1, self.peek1().map(|(_, x)| x)),
                ending(self.peek().1, self.peek1().map(|(_, x)| x)),
            ) {
                (_, Some(skip), _) => {
                    self.with_skip(skip);
                }
                (_, _, Some(s)) => {
                    skip = s;
                    break;
                }
                ((_, Token::Eof), _, _) => {
                    // Let the check at the top handle it.
                }
                ((span, found), _, _) => {
                    let _ = self.advance();
                    self.error_expected_found(span, expected_separator, found.description());
                    parses.push_back(make_error(self, span));
                    self.skip_until2(|tok, tok1| separator(tok, tok1).or(ending(tok, tok1)));
                }
            }
        }
        (skip, start.to(end), parses)
    }

    fn with_skip(&mut self, skip: Skip) -> Span {
        let (span, _) = self.advance();
        match skip {
            Skip::One => span,
            Skip::Two => {
                let (span1, _) = self.advance();
                span.to(span1)
            }
        }
    }

    fn parse_term(&mut self, allow_nl: bool) -> Result<Expr, Span> {
        self.maybe_skip_nls(allow_nl);
        match self.peek() {
            (span, Token::Keyword(b @ (Keyword::False | Keyword::True))) => {
                let _ = self.advance();
                Ok(Expr::new(
                    self.gcx,
                    ExprKind::Bool(b == Keyword::True),
                    span,
                ))
            }
            (span, Token::String(s)) => {
                let _ = self.advance();
                Ok(Expr::new(self.gcx, ExprKind::String(s), span))
            }
            (span, Token::Numeral(num, meta)) => {
                let _ = self.advance();
                Ok(Expr::new(self.gcx, ExprKind::Numeral(num, meta), span))
            }
            (span, Token::Ident(symbol)) => {
                let _ = self.advance();
                Ok(Expr::new(
                    self.gcx,
                    ExprKind::Ident(Ident { symbol, span }),
                    span,
                ))
            }
            (start, Token::LeftParen) => {
                let _ = self.advance();
                self.skip_nls();
                if let Some(end) = self.try_consume(Token::RightParen) {
                    Ok(Expr::new(self.gcx, ExprKind::Unit, start.to(end)))
                } else {
                    let expr = self.parse_expr(true)?;
                    let end = self.expect_matching(Token::RightParen, Token::LeftParen, start)?;
                    Ok(Expr::new(self.gcx, expr.get(self.gcx).kind, start.to(end)))
                }
            }
            (start, Token::LeftCurly) => {
                let _ = self.advance();
                self.skip_nls();
                let (_, fields) = self.parse_delimited(
                    Parser::parse_struct_field,
                    |tok| matches!(tok, Token::RightCurly),
                    |tok| matches!(tok, Token::Comma),
                    |this, x| {
                        x.map_or(Span::new_dummy(), |(id, ty)| {
                            id.span.to(ty.get(this.gcx).span)
                        })
                    },
                    |_, _| None,
                    Token::Comma.description(),
                    Token::RightCurly.description(),
                    Some((Token::LeftCurly.description(), start)),
                    true,
                );
                // rcurly
                let (end, _) = self.advance();

                Ok(Expr::new(
                    self.gcx,
                    ExprKind::StructLiteral(fields.into_iter().flatten().collect()),
                    start.to(end),
                ))
            }
            (start, Token::Keyword(Keyword::Do)) => {
                let _ = self.advance();
                let (_, stmts) = self.parse_stmts_block(
                    |tok| matches!(tok, Token::Keyword(Keyword::End)),
                    "`;`, newline, or `end`",
                    Token::Keyword(Keyword::End).description(),
                    Token::Keyword(Keyword::Do).description(),
                    start,
                );
                // parse_stmts_block ensures delimiter exists
                let (end, _) = self.advance();
                Ok(Expr::new(self.gcx, ExprKind::Do { stmts }, start.to(end)))
            }
            (span, found) => {
                self.error_expected_found(span, "expression", found.description());
                Err(span)
            }
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn parse_call(&mut self, allow_nl: bool) -> Result<Expr, Span> {
        let mut term = self.parse_term(allow_nl).or_expr_error(self);
        while let Some(lparen) = self.try_consume(Token::LeftParen) {
            let (_, exprs) = self.parse_exprs_list(
                |tok| matches!(tok, Token::RightParen),
                "`;` or `)`",
                Token::RightParen.description(),
                Token::LeftParen.description(),
                lparen,
            );
            // parse_exprs_list ensures delimiter exists
            let (end, _) = self.advance();
            term = Expr::new(
                self.gcx,
                ExprKind::Call(term, exprs),
                term.get(self.gcx).span.to(end),
            );
        }
        Ok(term)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn parse_field_access(&mut self, allow_nl: bool) -> Result<Expr, Span> {
        let start = self.span();
        let mut lhs = self.parse_call(allow_nl).or_expr_error(self);
        while self.try_consume(Token::Dot).is_some() {
            self.skip_nls();
            let rhs = match self.expect_ident() {
                Ok(rhs) => rhs,
                Err(rhs) => {
                    let _ = self.advance();
                    lhs = Expr::new(self.gcx, ExprKind::Error, start.to(rhs));
                    continue;
                }
            };

            lhs = Expr::new(
                self.gcx,
                ExprKind::FieldAccess(lhs, rhs),
                start.to(rhs.span),
            );
        }
        Ok(lhs)
    }

    fn parse_pratt(&mut self, allow_nl: bool, min_bp: u8) -> Expr {
        let mut lhs = if let Some(op) = self.peek().1.as_un_op() {
            let (span, _) = self.advance();
            self.skip_nls();
            let ((), r_bp) = prefix_binding_power(op);
            let rhs = self.parse_pratt(allow_nl, r_bp);
            Expr::new(
                self.gcx,
                ExprKind::UnaryOp {
                    kind: op,
                    op_span: span,
                    rhs,
                },
                span.to(rhs.get(self.gcx).span),
            )
        } else {
            self.parse_field_access(allow_nl).or_expr_error(self)
        };

        loop {
            let Some(op) = self.peek().1.as_bin_op() else {
                return lhs;
            };
            let op_span = self.peek().0;

            let (l_bp, r_bp) = infix_binding_power(op);
            if l_bp < min_bp {
                break;
            }

            // Advance over token
            let _ = self.advance();
            self.skip_nls();
            let rhs = self.parse_pratt(allow_nl, r_bp);

            lhs = Expr::new(
                self.gcx,
                ExprKind::BinaryOp {
                    lhs,
                    kind: op,
                    op_span,
                    rhs,
                },
                lhs.get(self.gcx).span.to(rhs.get(self.gcx).span),
            );
        }

        lhs
    }

    pub fn parse_expr(&mut self, allow_nl: bool) -> Result<Expr, Span> {
        self.maybe_skip_nls(allow_nl);
        // TODO: if-then-else
        Ok(self.parse_pratt(allow_nl, 0))
    }

    fn parse_ident_ty(&mut self) -> Option<(Ident, Ty)> {
        self.skip_nls();
        let id = self.expect_ident().ok()?;
        self.skip_nls();
        self.expect(Token::Colon).ok()?;
        self.skip_nls();
        let ty = self.parse_ty(true).or_ty_error(self);
        self.skip_nls();
        Some((id, ty))
    }

    fn parse_struct_field(&mut self) -> Option<(Ident, Expr)> {
        self.skip_nls();
        let id = self.expect_ident().ok()?;
        self.skip_nls();
        self.expect(Token::Colon).ok()?;
        self.skip_nls();
        let ty = self.parse_expr(false).or_expr_error(self);
        self.skip_nls();
        Some((id, ty))
    }

    /// expects `{` to be consumed already. `Span` is the rcurly
    fn parse_struct_defn_like(&mut self, lcurly_span: Span) -> (Span, im::Vector<(Ident, Ty)>) {
        let (_, args) = self.parse_delimited(
            Parser::parse_ident_ty,
            |tok| matches!(tok, Token::RightCurly),
            |tok| matches!(tok, Token::Comma),
            |this, x| {
                x.map_or(Span::new_dummy(), |(id, ty)| {
                    id.span.to(ty.get(this.gcx).span)
                })
            },
            |_, _| None,
            Token::Comma.description(),
            Token::RightCurly.description(),
            Some((Token::LeftCurly.description(), lcurly_span)),
            true,
        );
        // rcurly
        let (end, _) = self.advance();

        (end, args.into_iter().flatten().collect())
    }

    #[allow(clippy::too_many_lines)]
    fn parse_item(&mut self) -> Result<Item, Span> {
        match self.peek() {
            (start, Token::Keyword(Keyword::Fn)) => {
                let _ = self.advance();
                self.skip_nls();
                let ident = self.expect_ident().map_err(|sp| start.to(sp))?;
                self.skip_nls();
                let lparen = self.expect(Token::LeftParen).map_err(|sp| start.to(sp))?;
                self.skip_nls();
                let args = self.parse_delimited(
                    Parser::parse_ident_ty,
                    |tok| matches!(tok, Token::RightParen),
                    |tok| matches!(tok, Token::Comma),
                    |this, x| {
                        x.map_or(Span::new_dummy(), |(id, ty)| {
                            id.span.to(ty.get(this.gcx).span)
                        })
                    },
                    |_, _| None,
                    Token::Comma.description(),
                    Token::RightParen.description(),
                    Some((Token::LeftParen.description(), lparen)),
                    true,
                );
                // rparen
                let (_, _) = self.advance();
                let add_ret = self.span();
                self.skip_nls();
                let ret_ty = if self.try_consume(Token::Arrow).is_some() {
                    self.skip_nls();
                    Some(self.parse_ty(true).or_ty_error(self))
                } else {
                    None
                };
                self.skip_nls();
                let (delimiter, tok) = self.peek();
                self.skip_nls();
                let body = match tok {
                    Token::Eq => {
                        let _ = self.advance();
                        self.skip_nls();
                        self.parse_expr(false).or_expr_error(self)
                    }
                    Token::Keyword(Keyword::Do) => {
                        let _ = self.advance();
                        let (span, stmts) = self.parse_stmts_block(
                            |tok| matches!(tok, Token::Keyword(Keyword::End)),
                            "`;`, newline, or `end`",
                            Token::Keyword(Keyword::End).description(),
                            Token::Keyword(Keyword::Do).description(),
                            delimiter,
                        );
                        let _ = self.advance();
                        Expr::new(self.gcx, ExprKind::Do { stmts }, delimiter.to(span))
                    }
                    _ => {
                        self.error_expected_found(delimiter, "`=` or `do`", tok.description());
                        Expr::new(self.gcx, ExprKind::Error, start.until(delimiter))
                    }
                };

                // TODO: pub
                Ok(Item::new(
                    self.gcx,
                    ItemKind::Function {
                        is_pub: false,
                        name: ident,
                        args: args.1.into_iter().flatten().collect(),
                        ret_ty,
                        add_ret,
                        body,
                    },
                    start.to(body.get(self.gcx).span),
                ))
            }
            (start, Token::Keyword(Keyword::Type)) => {
                let _ = self.advance();
                self.skip_nls();
                let ident = self.expect_ident().map_err(|sp| start.to(sp))?;
                self.skip_nls();
                let _ = self.expect(Token::Eq).map_err(|sp| start.to(sp))?;
                self.skip_nls();
                match (self.peek(), self.peek1()) {
                    ((lcurly, Token::LeftCurly), _) => {
                        let _ = self.advance();
                        let (end, fields) = self.parse_struct_defn_like(lcurly);
                        Ok(Item::new(
                            self.gcx,
                            ItemKind::TypeDefn {
                                name: ident,
                                kind: TypeDefnKind::Struct { fields },
                            },
                            start.to(end),
                        ))
                    }
                    // TODO: make this more robust? (ident -> newline(s) -> pipe, for example)
                    ((_, first @ Token::Pipe), _)
                    | ((_, first @ Token::Ident(_)), Some((_, Token::LeftCurly | Token::Pipe))) => {
                        if matches!(first, Token::Pipe) {
                            let _ = self.advance();
                        }
                        let (skip, _, variants) = self.parse_delimited2(
                            |this| {
                                let start = this.span();
                                let ident = this.expect_ident().ok()?;
                                let (variant, span) =
                                    if let Some(lcurly) = this.try_consume(Token::LeftCurly) {
                                        let (end, fields) = this.parse_struct_defn_like(lcurly);
                                        (VariantKind::Struct(fields), start.to(end))
                                    } else {
                                        (VariantKind::Unit, start.to(ident.span))
                                    };
                                Some((ident, variant, span))
                            },
                            |tok, tok1| {
                                (matches!(tok, Token::Nl | Token::Eof)
                                    && !matches!(tok1, Some(Token::Pipe)))
                                .then_some(Skip::One)
                            },
                            |tok, tok1| {
                                if tok == Token::Pipe {
                                    Some(Skip::One)
                                } else if let (Token::Nl, Some(Token::Pipe)) = (tok, tok1) {
                                    Some(Skip::Two)
                                } else {
                                    None
                                }
                            },
                            |_this, variant| {
                                variant
                                    .clone()
                                    .map_or(Span::new_dummy(), |(_, _, span)| span)
                            },
                            |_this, _span| None,
                            Token::Pipe.description(),
                            "newline or end-of-file",
                            None,
                            false,
                        );
                        let end = self.with_skip(skip);
                        let variants = variants
                            .into_iter()
                            .flatten()
                            .map(|(i, x, _)| (i, x))
                            .collect();

                        Ok(Item::new(
                            self.gcx,
                            ItemKind::TypeDefn {
                                name: ident,
                                kind: TypeDefnKind::Sum { variants },
                            },
                            start.to(end),
                        ))
                    }
                    _ => todo!(),
                }
            }
            (span, found) => {
                self.error_expected_found(span, "item", found.description());
                Err(span)
            }
        }
    }

    fn is_item_start(&self) -> bool {
        matches!(self.peek().1, Token::Keyword(Keyword::Fn | Keyword::Type))
    }

    pub fn parse_top(&mut self) -> Vec<Item> {
        let mut items = vec![];
        self.skip_nls();
        if self.try_consume(Token::Eof).is_some() {
            return items;
        }
        while self.is_item_start() {
            self.skip_nls();
            items.push(self.parse_item().or_item_error(self));
            self.skip_nls();
            if self.try_consume(Token::Eof).is_some() {
                return items;
            }
        }
        items.push(self.parse_item().or_item_error(self));
        items
    }
}

impl Token {
    fn as_un_op(self) -> Option<UnOpKind> {
        Some(match self {
            Token::Not => UnOpKind::Not,
            Token::Minus => UnOpKind::Neg,
            _ => return None,
        })
    }

    fn as_bin_op(self) -> Option<BinOpKind> {
        Some(match self {
            Token::StarStar => BinOpKind::Power,
            Token::Star => BinOpKind::Multiply,
            Token::Slash => BinOpKind::Divide,
            Token::Percent => BinOpKind::Modulo,
            Token::Plus => BinOpKind::Add,
            Token::Minus => BinOpKind::Subtract,
            Token::Shl => BinOpKind::BitShiftLeft,
            Token::Shr => BinOpKind::BitShiftRight,
            Token::And => BinOpKind::BitAnd,
            Token::Pipe => BinOpKind::BitOr,
            Token::Xor => BinOpKind::BitXor,
            Token::EqEq => BinOpKind::Equal,
            Token::NotEq => BinOpKind::NotEqual,
            Token::Lt => BinOpKind::LessThan,
            Token::LtEq => BinOpKind::LessEqual,
            Token::Gt => BinOpKind::GreaterThan,
            Token::GtEq => BinOpKind::GreaterEqual,
            Token::BoolAnd => BinOpKind::LogicalAnd,
            Token::BoolOr => BinOpKind::LogicalOr,
            Token::Eq => BinOpKind::Assign,
            _ => return None,
        })
    }
}

fn infix_binding_power(op: BinOpKind) -> (u8, u8) {
    match op {
        BinOpKind::Power => (121, 120),
        BinOpKind::Multiply | BinOpKind::Divide | BinOpKind::Modulo => (100, 101),
        BinOpKind::Add | BinOpKind::Subtract => (90, 91),
        // TODO: I know bitwise precedences are sometimes
        // weird. Review this.
        BinOpKind::BitShiftLeft | BinOpKind::BitShiftRight => (80, 81),
        BinOpKind::BitAnd => (70, 71),
        BinOpKind::BitXor => (60, 61),
        BinOpKind::BitOr => (50, 51),
        // TODO: pass on AST to weed out nonassoc equal? or maybe
        // special-case a help note in typeck for e.g `1 <= x <= 2`
        BinOpKind::Equal
        | BinOpKind::NotEqual
        | BinOpKind::LessThan
        | BinOpKind::LessEqual
        | BinOpKind::GreaterThan
        | BinOpKind::GreaterEqual => (40, 41),
        // TODO: nonassoc. equal-precedence binops?
        BinOpKind::LogicalAnd => (30, 31),
        BinOpKind::LogicalOr => (20, 21),
        BinOpKind::Assign => (11, 10),
    }
}

fn prefix_binding_power(op: UnOpKind) -> ((), u8) {
    match op {
        UnOpKind::Neg | UnOpKind::Not => ((), 110),
    }
}

trait ParserExprResultExt<'gcx, I: Iterator<Item = SpanTok>> {
    fn or_expr_error(self, parser: &Parser<'gcx, I>) -> Expr;
}

impl<'gcx, I: Iterator<Item = SpanTok>> ParserExprResultExt<'gcx, I> for Result<Expr, Span> {
    fn or_expr_error(self, parser: &Parser<'gcx, I>) -> Expr {
        self.unwrap_or_else(|span| Expr::new(parser.gcx, ExprKind::Error, span))
    }
}

trait ParserTyResultExt<'gcx, I: Iterator<Item = SpanTok>> {
    fn or_ty_error(self, parser: &Parser<'gcx, I>) -> Ty;
}

impl<'gcx, I: Iterator<Item = SpanTok>> ParserTyResultExt<'gcx, I> for Result<Ty, Span> {
    fn or_ty_error(self, parser: &Parser<'gcx, I>) -> Ty {
        self.unwrap_or_else(|span| Ty::new(parser.gcx, TyKind::Error, span))
    }
}

trait ParserItemResultExt<'gcx, I: Iterator<Item = SpanTok>> {
    fn or_item_error(self, parser: &Parser<'gcx, I>) -> Item;
}

impl<'gcx, I: Iterator<Item = SpanTok>> ParserItemResultExt<'gcx, I> for Result<Item, Span> {
    fn or_item_error(self, parser: &Parser<'gcx, I>) -> Item {
        self.unwrap_or_else(|span| Item::new(parser.gcx, ItemKind::Error, span))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Skip {
    One,
    Two,
}

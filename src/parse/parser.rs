#![allow(clippy::explicit_auto_deref)]
use std::{borrow::Cow, iter};

use ariadne::{Color, Fmt, Label, ReportBuilder};
use chumsky::{
    error::{Error, RichReason},
    extra::Full,
    input::{BoxedStream, MapExtra, SpannedInput},
    pratt::{infix, left, prefix, right},
    prelude::*,
    util::MaybeRef,
};

use crate::{
    ast::{BinOpKind, Expr, ExprKind, GenericParam, Item, ItemKind, Numeral, Radix, Ty, TyKind},
    ctxt::GlobalCtxt,
    symbol::{kw::Keyword, special::EMPTY, Ident, Symbol},
};

use super::{
    lexer::{IdentLike, Token},
    Span, SpanWithFile,
};

pub type SyntaxError<'src> = Rich<'src, Token, Span>;
pub type CalInput<'src> = SpannedInput<Token, Span, BoxedStream<'src, (Token, Span)>>;
pub type Extra<'src> = Full<SyntaxError<'src>, &'src GlobalCtxt, bool>;

fn keyword(kw: Keyword) -> Token {
    Token::IdentLike(IdentLike::Keyword(kw))
}

fn ident<'src>() -> impl Parser<'src, CalInput<'src>, Ident, Extra<'src>> + Clone + 'src {
    any().try_map(|tok, span: Span| {
        if let Token::IdentLike(IdentLike::Ident(s)) = tok {
            Ok(Ident { symbol: s, span })
        } else {
            Err(<Rich<'_, _, _> as Error<'_, CalInput<'_>>>::expected_found(
                iter::once(Some(MaybeRef::Val(Token::IdentLike(IdentLike::Ident(
                    *EMPTY,
                ))))),
                Some(MaybeRef::Val(tok)),
                span,
            ))
        }
    })
}

fn under_ident<'src>() -> impl Parser<'src, CalInput<'src>, Ident, Extra<'src>> + Clone + 'src {
    just(Token::Under).map_with(|_, extra| Ident {
        symbol: Symbol::intern_static("_"),
        span: extra.span(),
    })
}

pub fn maybe_nls<'src>() -> impl Parser<'src, CalInput<'src>, (), Extra<'src>> + Clone + 'src {
    just(Token::Nl).ignored().repeated()
}

fn numeral<'src>() -> impl Parser<'src, CalInput<'src>, Expr, Extra<'src>> + Clone + 'src {
    any().try_map_with(|tok, extra| {
        let span = extra.span();
        if let Token::Numeral(num) = tok {
            Ok(Expr::new(*extra.state(), ExprKind::Numeral(num), span))
        } else {
            Err(<Rich<'_, _, _> as Error<'_, CalInput<'_>>>::expected_found(
                iter::once(Some(MaybeRef::Val(Token::Numeral(Numeral::Integer {
                    suffix: None,
                    radix: Radix::None,
                    sym: Symbol::intern_static("<numeral>"),
                })))),
                Some(MaybeRef::Val(tok)),
                span,
            ))
        }
    })
}

fn binop(lhs: Expr, op: Token, rhs: Expr, span: Span, gcx: &GlobalCtxt) -> Expr {
    Expr::new(
        gcx,
        ExprKind::BinaryOp {
            left: lhs,
            kind: match op {
                Token::StarStar => BinOpKind::Power,
                Token::Star => BinOpKind::Multiply,
                Token::Slash => BinOpKind::Divide,
                Token::Percent => BinOpKind::Modulo,
                Token::Plus => BinOpKind::Add,
                Token::Minus => BinOpKind::Subtract,
                Token::LtLt => BinOpKind::BitShiftLeft,
                Token::GtGt => BinOpKind::BitShiftRight,
                Token::And => BinOpKind::BitAnd,
                Token::Caret => BinOpKind::BitXor,
                Token::Pipe => BinOpKind::BitOr,
                Token::EqEq => BinOpKind::Equal,
                Token::BangEq => BinOpKind::NotEqual,
                Token::Lt => BinOpKind::LessThan,
                Token::Gt => BinOpKind::GreaterThan,
                Token::LtEq => BinOpKind::LessEqual,
                Token::GtEq => BinOpKind::GreaterEqual,
                Token::AndAnd => BinOpKind::LogicalAnd,
                Token::PipePipe => BinOpKind::LogicalOr,
                _ => unreachable!(),
            },
            right: rhs,
        },
        span,
    )
}

// TODO: check for where recovery should be done

pub fn ty<'src>() -> impl Parser<'src, CalInput<'src>, Ty, Extra<'src>> + Clone + 'src {
    recursive(|ty| {
        let ident = ident()
            .or(under_ident())
            .map_with(|ident, extra| Ty::new(*extra.state(), TyKind::Ident(ident), extra.span()));

        let func = just(keyword(Keyword::Fn))
            .then_ignore(maybe_nls())
            .ignore_then(just(Token::LParen))
            .then_ignore(maybe_nls())
            .ignore_then(
                ty.clone()
                    .then_ignore(maybe_nls())
                    .separated_by(just(Token::Comma).then_ignore(maybe_nls()))
                    .allow_trailing()
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(Token::RParen).then_ignore(maybe_nls()))
            .then(
                just(Token::Colon)
                    .then_ignore(maybe_nls())
                    .ignore_then(ty.clone())
                    .then_ignore(maybe_nls())
                    .or_not(),
            )
            .map_with(|(args, ret), extra| {
                Ty::new(
                    *extra.state(),
                    TyKind::Function(args.into(), ret),
                    extra.span(),
                )
            })
            .or(ident);

        func
    })
}

pub fn item<'src>(
    file: Symbol,
) -> impl Parser<'src, CalInput<'src>, Item, Extra<'src>> + Clone + 'src {
    let mut stmt = Recursive::declare();
    let mut expr = Recursive::declare();
    let mut item = Recursive::declare();

    item.define({
        let name_ty = ident().or(under_ident()).then_ignore(maybe_nls()).then(
            just(Token::Colon)
                .then_ignore(maybe_nls())
                .ignore_then(ty())
                .then_ignore(maybe_nls())
                .labelled("type annotation"),
        );

        just(keyword(Keyword::Fn))
            .ignore_then(maybe_nls())
            .ignore_then(ident())
            .then_ignore(maybe_nls())
            .then(
                just(Token::LBracket)
                    .ignore_then(
                        ident()
                            .map_with(|x, extra| GenericParam::new(*extra.state(), x))
                            .separated_by(just(Token::Comma).then(maybe_nls()))
                            .allow_trailing()
                            .collect::<Vec<_>>()
                            .map(im::Vector::from),
                    )
                    .then_ignore(just(Token::RBracket))
                    .or_not(),
            )
            .then_ignore(just(Token::LParen))
            .then_ignore(maybe_nls())
            .then(
                name_ty
                    .separated_by(just(Token::Comma).then_ignore(maybe_nls()))
                    .allow_trailing()
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(Token::RParen))
            .then_ignore(maybe_nls())
            .then(
                just(Token::Colon)
                    .then_ignore(maybe_nls())
                    .ignore_then(ty())
                    .then_ignore(maybe_nls())
                    .or_not(),
            )
            .then_ignore(maybe_nls())
            .then_ignore(just(Token::Arrow))
            .then(expr.clone())
            .map_with(move |((((name, generics), args), ret_ty), body), extra| {
                Item::new(
                    *extra.state(),
                    ItemKind::Function {
                        name,
                        generics: generics.unwrap_or_default(),
                        args: args.into(),
                        ret_ty,
                        body,
                    },
                    (file, extra.span()).into(),
                )
            })
    });

    stmt.define({
        let item = item
            .clone()
            .map_with(|item, extra| {
                let gcx = *extra.state();
                Expr::new(
                    gcx,
                    ExprKind::ItemStmt(item),
                    gcx.arenas.ast.item(item).span.1,
                )
            })
            .boxed();

        let let_binding = just(keyword(Keyword::Let))
            .then_ignore(maybe_nls())
            .ignore_then(
                just(keyword(Keyword::Mut))
                    .then_ignore(maybe_nls())
                    .or_not()
                    .map(|x| x.is_some())
                    .then(ident().or(under_ident()))
                    .then_ignore(maybe_nls())
                    .then(
                        just(Token::Colon)
                            .then_ignore(maybe_nls())
                            .ignore_then(ty())
                            .then_ignore(maybe_nls())
                            .or_not(),
                    )
                    .then_ignore(just(Token::Eq))
                    .then_ignore(maybe_nls())
                    // TODO: should this be `pratt`?
                    .then(expr.clone().with_ctx(false))
                    .map(|(((is_mut, ident), ty), expr)| (is_mut, ident, ty, expr)),
            )
            .map_with(|(is_mut, name, ty, val), extra| {
                let span = extra.span();
                Expr::new(
                    extra.state(),
                    ExprKind::Let {
                        is_mut,
                        name,
                        ty,
                        val,
                    },
                    span,
                )
            })
            .boxed();

        item.or(let_binding).or(expr.clone().with_ctx(false))
    });

    expr.define({
        let nls_with_context = just(Token::Nl)
            .ignored()
            .repeated()
            .configure(|cfg, ctx: &bool| if *ctx { cfg } else { cfg.exactly(0) });
        let stmts_block = stmt
            .clone()
            .separated_by(choice((
                just(Token::Semi).ignored(),
                just(Token::Nl).repeated().at_least(1).ignored(),
            )))
            .allow_trailing()
            .collect::<Vec<Expr>>()
            .map(im::Vector::from);

        let primary = choice((
            numeral(),
            ident().map_with(|ident, extra| {
                let span = extra.span();
                Expr::new(extra.state(), ExprKind::Ident(ident), span)
            }),
            one_of([keyword(Keyword::True), keyword(Keyword::False)]).map_with(|val, extra| {
                let span = extra.span();
                Expr::new(
                    *extra.state(),
                    ExprKind::Bool(val == keyword(Keyword::True)),
                    span,
                )
            }),
        ))
        .boxed()
        .labelled("literal");

        let term = choice((
            primary,
            expr.clone().with_ctx(true).delimited_by(
                just(Token::LParen).then_ignore(maybe_nls()),
                maybe_nls().ignore_then(just(Token::RParen)),
            ),
            just(keyword(Keyword::Do))
                .ignore_then(maybe_nls())
                .ignore_then(stmts_block.clone())
                .then_ignore(just(keyword(Keyword::End)))
                .map_with(|exprs, extra| {
                    let span = extra.span();
                    Expr::new(extra.state(), ExprKind::Do { exprs }, span)
                })
                .labelled("block"),
        ));

        let call = term.foldl_with(
            just(Token::LParen)
                .then_ignore(maybe_nls())
                .ignore_then(
                    expr.clone()
                        .separated_by(just(Token::Comma).then_ignore(maybe_nls()))
                        .allow_trailing()
                        .collect::<Vec<_>>()
                        .map(im::Vector::from),
                )
                .then_ignore(just(Token::RParen))
                .then_ignore(maybe_nls())
                .repeated(),
            |x, acc, extra| Expr::new(*extra.state(), ExprKind::Call(x, acc), extra.span()),
        );

        let pratt = call
            .pratt((
                infix(
                    right(120),
                    just(Token::StarStar).delimited_by(nls_with_context, maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), *extra.state())
                    },
                ),
                prefix(
                    110,
                    nls_with_context
                        .ignore_then(one_of([Token::Minus, Token::Bang]))
                        .then_ignore(maybe_nls()),
                    |op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        let span = extra.span();
                        Expr::new(
                            *extra.state(),
                            match op {
                                Token::Minus => ExprKind::UnaryMinus(rhs),
                                Token::Bang => ExprKind::UnaryNot(rhs),
                                _ => unreachable!(),
                            },
                            span,
                        )
                    },
                ),
                infix(
                    left(100),
                    one_of([Token::Star, Token::Slash, Token::Percent])
                        .delimited_by(nls_with_context, maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), *extra.state())
                    },
                ),
                infix(
                    left(90),
                    nls_with_context
                        .ignore_then(one_of([Token::Plus, Token::Minus]))
                        .then_ignore(maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), *extra.state())
                    },
                ),
                infix(
                    left(80),
                    one_of([Token::LtLt, Token::GtGt]).delimited_by(nls_with_context, maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), *extra.state())
                    },
                ),
                infix(
                    left(70),
                    just(Token::And).delimited_by(nls_with_context, maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), *extra.state())
                    },
                ),
                infix(
                    left(60),
                    just(Token::Caret).delimited_by(nls_with_context, maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), *extra.state())
                    },
                ),
                infix(
                    left(50),
                    just(Token::Pipe).delimited_by(nls_with_context, maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), *extra.state())
                    },
                ),
                infix(
                    left(40),
                    one_of([
                        Token::EqEq,
                        Token::BangEq,
                        Token::Lt,
                        Token::LtEq,
                        Token::Gt,
                        Token::GtEq,
                    ])
                    .delimited_by(nls_with_context, maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), *extra.state())
                    },
                ),
                infix(
                    left(30),
                    just(Token::AndAnd).delimited_by(nls_with_context, maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), *extra.state())
                    },
                ),
                infix(
                    left(20),
                    just(Token::PipePipe).delimited_by(nls_with_context, maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), *extra.state())
                    },
                ),
            ))
            .boxed()
            .labelled("expression");

        let name_ty = ident().or(under_ident()).then_ignore(maybe_nls()).then(
            just(Token::Colon)
                .then_ignore(maybe_nls())
                .ignore_then(ty())
                .then_ignore(maybe_nls())
                .labelled("type annotation")
                .or_not(),
        );

        let func = just(keyword(Keyword::Fn))
            .then_ignore(maybe_nls())
            .then_ignore(just(Token::LParen))
            .then_ignore(maybe_nls())
            .ignore_then(
                name_ty
                    .separated_by(just(Token::Comma).then_ignore(maybe_nls()))
                    .allow_trailing()
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(Token::RParen))
            .then_ignore(maybe_nls())
            .then(
                just(Token::Colon)
                    .then_ignore(maybe_nls())
                    .ignore_then(ty())
                    .then_ignore(maybe_nls())
                    .or_not(),
            )
            .then_ignore(maybe_nls())
            .then_ignore(just(Token::Arrow))
            .then(expr.clone())
            .map_with(move |((args, ret_ty), body), extra| {
                Expr::new(
                    *extra.state(),
                    ExprKind::Closure {
                        args: args.into(),
                        ret_ty,
                        body,
                    },
                    extra.span(),
                )
            })
            .boxed()
            .or(pratt);

        func
    });

    item.then_ignore(maybe_nls())
}

pub fn render_diagnostic(
    e: &RichReason<'_, Token>,
    span: Span,
    file: Symbol,
    mut report: ReportBuilder<'static, SpanWithFile>,
) -> ReportBuilder<'static, SpanWithFile> {
    match e {
        RichReason::Custom(msg) => report.with_message(msg).with_label(
            Label::new(SpanWithFile(file, span))
                .with_message(format!("{}", msg.fg(Color::Red)))
                .with_color(Color::Red),
        ),
        RichReason::ExpectedFound { expected, found } => report
            .with_message(format!(
                "{}, expected: {}",
                if let Some(found) = found {
                    Cow::from(format!("Unexpected token {}", found.description()))
                } else {
                    Cow::from("Unexpected end of input")
                },
                if expected.is_empty() {
                    Cow::from("end of input")
                } else {
                    Cow::from(
                        expected
                            .iter()
                            .map(|x| match x {
                                chumsky::error::RichPattern::Token(tok) => {
                                    Cow::from(tok.description())
                                }
                                chumsky::error::RichPattern::Label(l) => Cow::from(*l),
                                chumsky::error::RichPattern::EndOfInput => {
                                    Cow::from("end of input")
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                },
            ))
            .with_label(
                Label::new(SpanWithFile(file, span))
                    .with_message("didn't expect this token".fg(Color::Red))
                    .with_color(Color::Red),
            ),
        RichReason::Many(vec) => {
            for reason in vec {
                report = render_diagnostic(reason, span, file, report);
            }
            report
        }
    }
}

use std::{iter, sync::Arc};

use chumsky::{
    error::Error,
    extra::Full,
    input::{BoxedStream, MapExtra, SpannedInput},
    pratt::{infix, left, prefix, right},
    prelude::*,
    util::MaybeRef,
};
use id_arena::Id;

use crate::{
    ast::{self, BinOpKind, Expr, ExprKind, Numeral, Radix, Ty, TyKind},
    ctxt::GlobalCtxt,
    symbol::{kw::Keyword, primitives::Primitive, special::EMPTY, Ident, Symbol},
};

use super::{
    lexer::{IdentLike, Token},
    Span,
};

pub type SyntaxError<'src> = Rich<'src, Token, Span>;
pub type CalInput<'src> = SpannedInput<Token, Span, BoxedStream<'src, (Token, Span)>>;
pub type Extra<'src> = Full<SyntaxError<'src>, Arc<GlobalCtxt>, ()>;

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

fn maybe_nls<'src>() -> impl Parser<'src, CalInput<'src>, (), Extra<'src>> + Clone + 'src {
    just(Token::Nl).ignored().repeated()
}

fn numeral<'src>() -> impl Parser<'src, CalInput<'src>, Id<Expr>, Extra<'src>> + Clone + 'src {
    any().try_map_with(|tok, extra| {
        let span = extra.span();
        if let Token::Numeral(num) = tok {
            Ok(Expr::new(extra.state(), ExprKind::Numeral(num), span))
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

fn binop(lhs: Id<Expr>, op: Token, rhs: Id<Expr>, span: Span, gcx: &Arc<GlobalCtxt>) -> Id<Expr> {
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

pub fn ty<'src>() -> impl Parser<'src, CalInput<'src>, Id<Ty>, Extra<'src>> + Clone + 'src {
    select! {
	Token::IdentLike(IdentLike::Primitive(Primitive::Uint)) => TyKind::Primitive(ast::Primitive::Uint),
	Token::IdentLike(IdentLike::Primitive(Primitive::Bool)) => TyKind::Primitive(ast::Primitive::Bool),
    }.map_with(|kind, extra| {
	let span = extra.span();
	Ty::new(extra.state(), kind, span)
    })
}

pub fn expr<'src>() -> impl Parser<'src, CalInput<'src>, Id<Expr>, Extra<'src>> + Clone + 'src {
    let expr = recursive(|expr| {
        let exprs_block = expr
            .clone()
            .separated_by(choice((
                just(Token::Semi).ignored(),
                just(Token::Nl).repeated().at_least(1).ignored(),
            )))
            .allow_trailing()
            .collect::<Vec<Id<Expr>>>()
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
                    extra.state(),
                    ExprKind::Bool(val == keyword(Keyword::True)),
                    span,
                )
            }),
        ))
        .boxed()
        .labelled("literal");

        let term = choice((
            primary,
            expr.clone().delimited_by(
                just(Token::LParen).then_ignore(maybe_nls()),
                maybe_nls().ignore_then(just(Token::RParen)),
            ),
            just(keyword(Keyword::Do))
                .ignore_then(maybe_nls())
                .ignore_then(exprs_block.clone())
                .then_ignore(just(keyword(Keyword::End)))
                .map_with(|exprs, extra| {
                    let span = extra.span();
                    Expr::new(extra.state(), ExprKind::Do { exprs }, span)
                })
                .labelled("block"),
        ));

        let pratt = term
            .pratt((
                infix(
                    right(120),
                    just(Token::StarStar).delimited_by(maybe_nls(), maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), extra.state())
                    },
                ),
                prefix(
                    110,
                    one_of([Token::Minus, Token::Bang]).delimited_by(maybe_nls(), maybe_nls()),
                    |op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        let span = extra.span();
                        Expr::new(
                            extra.state(),
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
                        .delimited_by(maybe_nls(), maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), extra.state())
                    },
                ),
                infix(
                    left(90),
                    one_of([Token::Plus, Token::Minus]).delimited_by(maybe_nls(), maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), extra.state())
                    },
                ),
                infix(
                    left(80),
                    one_of([Token::LtLt, Token::GtGt]).delimited_by(maybe_nls(), maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), extra.state())
                    },
                ),
                infix(
                    left(70),
                    just(Token::And).delimited_by(maybe_nls(), maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), extra.state())
                    },
                ),
                infix(
                    left(60),
                    just(Token::Caret).delimited_by(maybe_nls(), maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), extra.state())
                    },
                ),
                infix(
                    left(50),
                    just(Token::Pipe).delimited_by(maybe_nls(), maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), extra.state())
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
                    .delimited_by(maybe_nls(), maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), extra.state())
                    },
                ),
                infix(
                    left(30),
                    just(Token::AndAnd).delimited_by(maybe_nls(), maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), extra.state())
                    },
                ),
                infix(
                    left(20),
                    just(Token::PipePipe).delimited_by(maybe_nls(), maybe_nls()),
                    |lhs, op, rhs, extra: &mut MapExtra<'src, '_, _, _>| {
                        binop(lhs, op, rhs, extra.span(), extra.state())
                    },
                ),
            ))
            .boxed()
            .labelled("expression");

        let let_expr = just(keyword(Keyword::Let))
            .then_ignore(maybe_nls())
            .ignore_then(
                just(keyword(Keyword::Mut))
                    .then_ignore(maybe_nls())
                    .or_not()
                    .map(|x| x.is_some()),
            )
            .then(
                ident()
                    .or(under_ident())
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
                    .then(expr)
                    .then_ignore(maybe_nls())
                    .map(|((ident, ty), expr)| (ident, ty, expr))
                    .separated_by(just(Token::Comma).then_ignore(maybe_nls()))
                    .at_least(1)
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .map(im::Vector::from),
            )
            .then_ignore(maybe_nls())
            .then_ignore(just(keyword(Keyword::In)))
            .then_ignore(maybe_nls())
            .then(exprs_block)
            .then_ignore(just(keyword(Keyword::End)))
            .map_with(|((is_mut, varlist), in_block), extra| {
                let span = extra.span();
                Expr::new(
                    extra.state(),
                    ExprKind::Let {
                        is_mut,
                        varlist,
                        in_block,
                    },
                    span,
                )
            })
            .or(pratt);

        let_expr.labelled("expression")
    });

    expr.then_ignore(maybe_nls())
}

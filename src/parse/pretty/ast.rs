use pretty::{DocAllocator, RcAllocator, RcDoc};

use crate::ast::{BinOpKind, Expr, ExprKind, Item, ItemKind, Numeral, Ty, TyKind};

use super::Printer;

impl<'gcx> Printer<'gcx> {
    pub fn print_expr(&self, expr: Expr) -> RcDoc {
        let arena = &self.gcx.arenas.ast;
        match arena.expr(expr).kind {
            ExprKind::Let {
                is_mut,
                name,
                ty,
                val,
            } => {
                // TODO
                let varlist = vec![(is_mut, name, ty, val)];
                RcDoc::text("(")
                    .append("let")
                    .append(RcDoc::space())
                    .append(RcDoc::text("["))
                    .append(
                        RcDoc::intersperse(
                            varlist.into_iter().map(|(is_mut, var, ty, expr)| {
                                if is_mut {
                                    RcDoc::text("(mut").append(RcDoc::space())
                                } else {
                                    RcDoc::nil()
                                }
                                .append(RcDoc::text(var.as_str()))
                                .append(if is_mut {
                                    RcDoc::text(")")
                                } else {
                                    RcDoc::nil()
                                })
                                .append(if let Some(ty) = ty {
                                    RcDoc::space()
                                        .append(self.print_ty(ty))
                                        .nest((var.as_str().len() + 1) as isize)
                                } else {
                                    RcDoc::nil()
                                })
                                .append(
                                    RcDoc::space()
                                        .append(self.print_expr(expr))
                                        .nest((var.as_str().len() + 1) as isize),
                                )
                                .group()
                            }),
                            RcDoc::line(),
                        )
                        .nest(6),
                    )
                    .append(RcDoc::text("]"))
            }
            ExprKind::BinaryOp { left, kind, right } => {
                RcDoc::text(format!("({}", self.print_binopkind(kind)))
                    .append(
                        RcDoc::space()
                            .append(self.print_expr(left))
                            .append(RcDoc::line())
                            .append(self.print_expr(right))
                            .group()
                            .append(RcDoc::text(")")),
                    )
                    .nest((self.print_binopkind(kind).len() + 2) as isize)
            }
            ExprKind::UnaryMinus(expr) => RcDoc::text("(neg")
                .append(RcDoc::space().append(self.print_expr(expr)).nest(5))
                .append(RcDoc::text(")")),
            ExprKind::UnaryNot(expr) => RcDoc::text("(not")
                .append(RcDoc::space().append(self.print_expr(expr)).nest(5))
                .append(RcDoc::text(")")),
            ExprKind::Do { exprs } => RcDoc::text("(do").append(
                RcDoc::line()
                    .append(
                        RcDoc::intersperse(
                            exprs.into_iter().map(|expr| self.print_expr(expr).group()),
                            RcDoc::line(),
                        )
                        .append(RcDoc::text(")")),
                    )
                    .nest(4),
            ),
            ExprKind::Numeral(Numeral::Float { sym, .. } | Numeral::Integer { sym, .. }) => {
                RcDoc::text(sym.as_str())
            }
            ExprKind::Ident(ident) => RcDoc::text(ident.as_str()),
            ExprKind::Bool(b) => RcDoc::text(format!("{}", b)),
            ExprKind::Error => RcDoc::text("<error>"),
        }
    }

    fn print_binopkind(&self, kind: BinOpKind) -> &'static str {
        match kind {
            BinOpKind::LogicalOr => "||",
            BinOpKind::LogicalAnd => "&&",
            BinOpKind::BitOr => "|",
            BinOpKind::BitAnd => "&",
            BinOpKind::BitXor => "^",
            BinOpKind::Equal => "==",
            BinOpKind::NotEqual => "!=",
            BinOpKind::LessThan => "<",
            BinOpKind::GreaterThan => ">",
            BinOpKind::LessEqual => "<=",
            BinOpKind::GreaterEqual => ">=",
            BinOpKind::BitShiftLeft => "<<",
            BinOpKind::BitShiftRight => ">>",
            BinOpKind::Add => "+",
            BinOpKind::Subtract => "-",
            BinOpKind::Multiply => "*",
            BinOpKind::Divide => "/",
            BinOpKind::Modulo => "%",
            BinOpKind::Power => "**",
        }
    }

    pub fn print_ty(&self, ty: Ty) -> RcDoc {
        let arena = &self.gcx.arenas.ast;
        match arena.ty(ty).kind {
            TyKind::Ident(ident) => RcDoc::text(ident.as_str()),
            TyKind::Function(args, ret) => {
                let args = args.iter().map(|arg| self.print_ty(*arg));
                let ret = ret.map(|ret| {
                    RcAllocator
                        .text(":")
                        .append(RcDoc::space())
                        .append(self.print_ty(ret))
                });

                RcAllocator
                    .text("fn(")
                    .append(RcAllocator.intersperse(args, RcDoc::text(",").append(RcDoc::space())))
                    .append(")")
                    .append(ret)
                    .into_doc()
            }
        }
    }

    pub fn print_item(&self, item: Item) -> RcDoc {
        let arena = &self.gcx.arenas.ast;
        match arena.item(item).kind {
            ItemKind::Function {
                name,
                args,
                ret_ty,
                body,
            } => {
                let args = args.iter().map(|(name, ty)| {
                    RcAllocator
                        .text(name.as_str())
                        .append(":")
                        .append(RcDoc::space())
                        .append(self.print_ty(*ty))
                });

                let ret_ty = ret_ty.map(|ret_ty| {
                    RcDoc::text(":")
                        .append(RcDoc::space())
                        .append(self.print_ty(ret_ty))
                });

                RcAllocator
                    .text("fn")
                    .append(RcDoc::space())
                    .append(name.as_str())
                    .append("(")
                    .append(RcAllocator.intersperse(args, RcDoc::text(",").append(RcDoc::space())))
                    .append(")")
                    .append(ret_ty.unwrap_or(RcDoc::nil()))
                    .append(RcDoc::space())
                    .append("->")
                    .append(RcDoc::softline())
                    .append(
                        RcAllocator
                            .nil()
                            .append(self.print_expr(body))
                            .indent(4)
                            .group(),
                    )
                    .into_doc()
            }
        }
    }
}

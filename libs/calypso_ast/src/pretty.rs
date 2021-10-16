use std::fmt::{self, Display, Write};

use crate::{
    expr::{Expr, Mutability, Primary},
    traverse::Visitor,
    ty::Ty,
};
use calypso_base::span::Spanned;
use calypso_error::CalResult;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub struct PrettyPrinter(String);

impl Display for PrettyPrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Visitor for PrettyPrinter {
    fn visit_primary<'src>(&mut self, src: &'src str, x: Spanned<&Primary>) -> CalResult<()> {
        match *x.value() {
            Primary::Number(_) => write!(self.0, "{}", &src[x.span().into_range()])?,
            Primary::Bool(b) => write!(self.0, "{}", b)?,
            Primary::Symbol(sym) => write!(self.0, "{}", sym)?,
        }
        Ok(())
    }

    fn visit_expr<'src>(&mut self, src: &'src str, x: Spanned<&Expr>) -> CalResult<()> {
        match x.value() {
            Expr::BinOp(left, op, right) => {
                write!(self.0, "({} ", op.value())?;
                self.visit_expr(src, left.as_ref().map(AsRef::as_ref))?;
                write!(self.0, " ")?;
                self.visit_expr(src, right.as_ref().map(AsRef::as_ref))?;
                write!(self.0, ")")?;
            }
            Expr::Primary(primary) => self.visit_primary(src, primary.as_ref())?,
            Expr::UnOp(op, expr) => {
                write!(self.0, "({} ", op.value())?;
                self.visit_expr(src, expr.as_ref().map(AsRef::as_ref))?;
                write!(self.0, ")")?;
            }
            Expr::Block(exprs) => {
                write!(self.0, "(block")?;
                for expr in exprs {
                    write!(self.0, " ")?;
                    self.visit_expr(src, expr.as_ref())?;
                }
                write!(self.0, ")")?;
            }
            Expr::Let(is_mut, sym, ty, val, expr_in) => {
                write!(self.0, "(let ")?;
                if let Mutability::Mut = is_mut {
                    write!(self.0, "mut ")?;
                }
                write!(self.0, "{} ", sym.value_owned())?;
                if let Some(ty) = ty {
                    write!(self.0, "(ty ")?;
                    self.visit_ty(src, ty.as_ref())?;
                    write!(self.0, ") ")?;
                }
                self.visit_expr(src, val.as_ref().map(AsRef::as_ref))?;
                write!(self.0, " in ")?;
                self.visit_expr(src, expr_in.as_ref().map(AsRef::as_ref))?;
                write!(self.0, ")")?;
            }
        }
        Ok(())
    }

    fn visit_ty<'src>(&mut self, _src: &'src str, x: Spanned<&crate::ty::Ty>) -> CalResult<()> {
        match x.value() {
            Ty::Symbol(sym) => {
                write!(self.0, "{}", sym.value())?;
            }
        }
        Ok(())
    }
}

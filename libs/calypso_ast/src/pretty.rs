use std::fmt::{self, Display, Write};

use crate::{
    expr::{Expr, Primary},
    stmt::Stmt,
    traverse::Visitor,
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
        }
        Ok(())
    }

    fn visit_stmt<'src>(&mut self, src: &'src str, x: Spanned<&Stmt>) -> CalResult<()> {
        match x.value() {
            Stmt::Let(sym, expr) => {
                write!(self.0, "(let {} ", sym.value_owned())?;
                self.visit_expr(src, expr.as_ref())?;
                write!(self.0, ")")?;
            }
        }
        Ok(())
    }
}

use std::fmt::{self, Display, Write};

use crate::{
    expr::{Expr, Primary},
    traverse::Visitor,
};
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
    fn visit_primary(&mut self, x: &Primary) -> CalResult<()> {
        match *x {
            Primary::Number(num, radix, suffix) => write!(
                self.0,
                "{}{}{}",
                radix,
                num,
                suffix.map(|s| format!("{}", s)).unwrap_or_default()
            )?,
            Primary::Bool(b) => write!(self.0, "{}", b)?,
        }
        Ok(())
    }

    fn visit_expr(&mut self, x: &Expr) -> CalResult<()> {
        match x {
            Expr::BinOp(left, op, right) => {
                write!(self.0, "({} ", op)?;
                self.visit_expr(left)?;
                write!(self.0, " ")?;
                self.visit_expr(right)?;
                write!(self.0, ")")?;
            }
            Expr::Primary(primary) => self.visit_primary(primary)?,
            Expr::UnOp(op, expr) => {
                write!(self.0, "({} ", op.value())?;
                self.visit_expr(expr)?;
                write!(self.0, ")")?;
            }
        }
        Ok(())
    }
}

use crate::expr::{Expr, Primary};
use calypso_error::CalResult;

pub trait Visitor {
    /// Visit an expression.
    ///
    /// # Errors
    ///
    /// This function may arbitrarily error depending on its implementation.
    fn visit_expr<'src>(&mut self, _src: &'src str, _expr: &Expr) -> CalResult<()> {
        Ok(())
    }

    /// Visit an primary value.
    ///
    /// # Errors
    ///
    /// This function may arbitrarily error depending on its implementation.
    fn visit_primary<'src>(&mut self, _src: &'src str, _primary: &Primary) -> CalResult<()> {
        Ok(())
    }
}

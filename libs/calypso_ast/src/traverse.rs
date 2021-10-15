use crate::{
    expr::{Expr, Primary},
    stmt::Stmt,
};
use calypso_base::span::Spanned;
use calypso_error::CalResult;

pub trait Visitor {
    /// Visit a statement.
    ///
    /// # Errors
    ///
    /// This funciton may arbitrarily error depending on its implementation.]
    fn visit_stmt<'src>(&mut self, _src: &'src str, _stmt: Spanned<&Stmt>) -> CalResult<()> {
        Ok(())
    }

    /// Visit an expression.
    ///
    /// # Errors
    ///
    /// This function may arbitrarily error depending on its implementation.
    fn visit_expr<'src>(&mut self, _src: &'src str, _expr: Spanned<&Expr>) -> CalResult<()> {
        Ok(())
    }

    /// Visit a primary value.
    ///
    /// # Errors
    ///
    /// This function may arbitrarily error depending on its implementation.
    fn visit_primary<'src>(
        &mut self,
        _src: &'src str,
        _primary: Spanned<&Primary>,
    ) -> CalResult<()> {
        Ok(())
    }
}

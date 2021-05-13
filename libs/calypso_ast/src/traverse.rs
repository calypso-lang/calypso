use crate::expr::{Expr, Primary};
use calypso_error::CalResult;

pub trait Visitor {
    fn visit_expr(&mut self, _expr: &Expr) -> CalResult<()> {
        Ok(())
    }

    fn visit_primary(&mut self, _primary: &Primary) -> CalResult<()> {
        Ok(())
    }
}

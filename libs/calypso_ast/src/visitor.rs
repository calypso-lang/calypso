use std::cell::RefCell;
use std::rc::Rc;

use crate::expr::{Expr, Primary};
use calypso_error::CalResult;

pub trait Visitor {
    fn visit_expr(&mut self, _expr: Rc<RefCell<Expr>>) -> CalResult<()> {
        Ok(())
    }

    fn visit_primary(&mut self, _primary: Rc<RefCell<Primary>>) -> CalResult<()> {
        Ok(())
    }
}

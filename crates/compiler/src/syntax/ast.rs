use std::{cell::RefCell, fmt};

use crate::{
    arena::{Arena, IdLike},
    ctxt::GlobalCtxt,
    new_ast_ty,
    symbol::{Symbol, prim_ty::PrimitiveTy},
};

use super::span::Span;

#[derive(Debug, Default)]
pub struct AstArenas {
    pub ty: RefCell<Arena<Ty, TyData>>,
}

impl AstArenas {
    pub fn clear(&self) {
        self.ty.borrow_mut().clear();
    }
}

new_ast_ty!(Ty, TyData, ast, ty, kind: TyKind, span: Span);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TyKind {
    Primitive(PrimitiveTy),
    Array(Ty),
    Error,
}

struct DebugTy<'gcx>(Ty, &'gcx GlobalCtxt);

impl fmt::Debug for DebugTy<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.get(self.1).kind {
            TyKind::Primitive(prim) => write!(f, "{}", Symbol::from(prim)),
            TyKind::Array(ty) => write!(f, "{:?}", DebugTy(ty, self.1)),
            TyKind::Error => write!(f, "<error>"),
        }
    }
}

impl Ty {
    pub fn debug(self, gcx: &'_ GlobalCtxt) -> impl fmt::Debug + '_ {
        DebugTy(self, gcx)
    }
}

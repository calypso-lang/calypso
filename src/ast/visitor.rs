use crate::{ctxt::GlobalCtxt, symbol::Ident};

use super::{Expr, ExprKind, GenericParam, Item, ItemKind, Ty, TyKind};

pub trait AstVisitor: Sized {
    fn gcx(&'_ self) -> &'_ GlobalCtxt;

    fn visit_ident(&mut self, _ident: Ident) {}

    fn visit_expr(&mut self, expr: Expr) {
        walk_expr(self, expr)
    }

    fn visit_ty(&mut self, ty: Ty) {
        walk_ty(self, ty)
    }

    fn visit_item(&mut self, item: Item) {
        walk_item(self, item)
    }

    fn visit_generic_param(&mut self, param: GenericParam) {
        walk_generic_param(self, param)
    }
}

pub fn walk_expr<V: AstVisitor>(visitor: &mut V, expr: Expr) {
    match visitor.gcx().arenas.ast.expr(expr).kind {
        ExprKind::ItemStmt(item) => visitor.visit_item(item),
        ExprKind::Let {
            is_mut: _,
            name,
            ty,
            val,
        } => {
            visitor.visit_ident(name);
            if let Some(ty) = ty {
                visitor.visit_ty(ty);
            }
            visitor.visit_expr(val);
        }
        ExprKind::Closure { args, ret_ty, body } => {
            for (arg, ty) in args {
                visitor.visit_ident(arg);
                if let Some(ty) = ty {
                    visitor.visit_ty(ty);
                }
            }
            if let Some(ret_ty) = ret_ty {
                visitor.visit_ty(ret_ty);
            }
            visitor.visit_expr(body);
        }
        ExprKind::BinaryOp {
            left,
            kind: _,
            right,
        } => {
            visitor.visit_expr(left);
            visitor.visit_expr(right);
        }
        ExprKind::UnaryMinus(right) | ExprKind::UnaryNot(right) => {
            visitor.visit_expr(right);
        }
        ExprKind::Do { exprs } => {
            for expr in exprs {
                visitor.visit_expr(expr);
            }
        }
        ExprKind::Call(f, xs) => {
            visitor.visit_expr(f);
            for x in xs {
                visitor.visit_expr(x);
            }
        }
        ExprKind::Numeral(_) | ExprKind::Bool(_) | ExprKind::Error => {}
        ExprKind::Ident(ident) => {
            visitor.visit_ident(ident);
        }
    }
}

pub fn walk_ty<V: AstVisitor>(visitor: &mut V, ty: Ty) {
    match visitor.gcx().arenas.ast.ty(ty).kind {
        TyKind::Function(tys, ret_ty) => {
            for ty in tys {
                visitor.visit_ty(ty);
            }
            if let Some(ret_ty) = ret_ty {
                visitor.visit_ty(ret_ty);
            }
        }
        TyKind::Ident(ident) => visitor.visit_ident(ident),
    }
}

pub fn walk_item<V: AstVisitor>(visitor: &mut V, item: Item) {
    match visitor.gcx().arenas.ast.item(item).kind {
        ItemKind::Function {
            name,
            generics,
            args,
            ret_ty,
            body,
        } => {
            visitor.visit_ident(name);
            for generic in generics {
                visitor.visit_generic_param(generic);
            }
            for (arg, ty) in args {
                visitor.visit_ident(arg);
                visitor.visit_ty(ty);
            }
            if let Some(ret_ty) = ret_ty {
                visitor.visit_ty(ret_ty);
            }
            visitor.visit_expr(body);
        }
    }
}

pub fn walk_generic_param<V: AstVisitor>(visitor: &mut V, param: GenericParam) {
    visitor.visit_ident(param.ident);
}

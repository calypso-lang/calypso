use std::{cell::RefCell, fmt};

use crate::{
    arena::{Arena, IdLike},
    ctxt::GlobalCtxt,
    new_ast_ty,
    symbol::{Ident, Symbol, prim_ty::PrimitiveTy},
};

use super::{span::Span, token::Numeral};

#[derive(Debug, Default)]
pub struct AstArenas {
    pub ty: RefCell<Arena<Ty, TyData>>,
    pub expr: RefCell<Arena<Expr, ExprData>>,
    pub item: RefCell<Arena<Item, ItemData>>,
    pub pattern: RefCell<Arena<Pattern, PatternData>>,
}

impl AstArenas {
    pub fn clear(&self) {
        self.ty.borrow_mut().clear();
        self.expr.borrow_mut().clear();
        self.item.borrow_mut().clear();
        self.pattern.borrow_mut().clear();
    }
}

new_ast_ty!(Ty, TyData, ast, ty, kind: TyKind, span: Span);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TyKind {
    Primitive(PrimitiveTy),
    Array(Ty),
    Option(Ty),
    // TODO: this will be refactored into tuples
    Unit,
    Modal(Modality, Ty),
    Ident(Ident),
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Modality {
    RefImm,
    RefMut,
    Ref,
    Unique,
    SharedImm,
    SharedMut,
}

struct DebugTy<'gcx>(Ty, &'gcx GlobalCtxt);

impl Ty {
    pub fn debug(self, gcx: &'_ GlobalCtxt) -> impl fmt::Debug + '_ {
        DebugTy(self, gcx)
    }
}

impl fmt::Debug for DebugTy<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.get(self.1).kind {
            TyKind::Primitive(prim) => write!(f, "{}", Symbol::from(prim)),
            TyKind::Array(ty) => f.debug_tuple("Array").field(&DebugTy(ty, self.1)).finish(),
            TyKind::Option(ty) => f.debug_tuple("Option").field(&DebugTy(ty, self.1)).finish(),
            TyKind::Modal(mode, ty) => f
                .debug_tuple("Modal")
                .field(&mode)
                .field(&DebugTy(ty, self.1))
                .finish(),
            TyKind::Unit => write!(f, "Unit"),
            TyKind::Error => write!(f, "Error"),
            TyKind::Ident(ident) => write!(f, "{}", ident.symbol),
        }
    }
}

new_ast_ty!(Expr, ExprData, ast, expr, kind: ExprKind, span: Span);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    // TODO: let-expressions?
    Let {
        name: Ident,
        is_mut: bool,
        ty: Option<Ty>,
        val: Expr,
    },
    BinaryOp {
        lhs: Expr,
        kind: BinOpKind,
        op_span: Span,
        rhs: Expr,
    },
    UnaryOp {
        kind: UnOpKind,
        op_span: Span,
        rhs: Expr,
    },
    Do {
        stmts: im::Vector<Expr>,
    },
    StructLiteral(im::Vector<(Ident, Expr)>),
    FieldAccess(Expr, Ident),
    // TODO: refactor this
    If(Expr, Expr, Expr),
    Match(Expr, im::Vector<(Pattern, Expr)>),
    Call(Expr, im::Vector<Expr>),
    Numeral(i128, Numeral),
    Ident(Ident),
    Bool(bool),
    String(Symbol),
    Unit,
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnOpKind {
    Not,
    Neg,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BinOpKind {
    LogicalOr,
    LogicalAnd,
    BitOr,
    BitAnd,
    BitXor,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    BitShiftLeft,
    BitShiftRight,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Assign,
}

struct DebugExpr<'gcx>(Expr, &'gcx GlobalCtxt);

impl Expr {
    pub fn debug(self, gcx: &'_ GlobalCtxt) -> impl fmt::Debug + '_ {
        DebugExpr(self, gcx)
    }
}

impl fmt::Debug for DebugExpr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.get(self.1).kind {
            ExprKind::Do { stmts } => {
                let mut f = f.debug_tuple("Do");
                for stmt in stmts {
                    f.field(&DebugExpr(stmt, self.1));
                }
                f.finish()
            }
            ExprKind::Bool(b) => write!(f, "{b:?}"),
            ExprKind::Let {
                name,
                is_mut,
                ty,
                val,
            } => f
                .debug_struct("Let")
                .field("name", &name)
                .field("is_mut", &is_mut)
                .field("ty", &ty.map(|ty| DebugTy(ty, self.1)))
                .field("val", &DebugExpr(val, self.1))
                .finish(),
            ExprKind::Error => write!(f, "Error"),
            ExprKind::Unit => write!(f, "Unit"),
            ExprKind::Call(func, args) => f
                .debug_tuple("Call")
                .field(&DebugExpr(func, self.1))
                .field(
                    &args
                        .into_iter()
                        .map(|x| DebugExpr(x, self.1))
                        .collect::<Vec<_>>(),
                )
                .finish(),
            ExprKind::Ident(id) => write!(f, "{}", id.symbol),
            ExprKind::BinaryOp { lhs, kind, rhs, .. } => f
                .debug_struct("BinaryOp")
                .field("lhs", &DebugExpr(lhs, self.1))
                .field("kind", &kind)
                .field("rhs", &DebugExpr(rhs, self.1))
                .finish(),
            ExprKind::UnaryOp { kind, rhs, .. } => f
                .debug_struct("UnaryOp")
                .field("kind", &kind)
                .field("rhs", &DebugExpr(rhs, self.1))
                .finish(),
            ExprKind::Numeral(num, Numeral::Integer { radix, suffix }) => {
                let neg = if num.is_negative() { "-" } else { "" };
                let num_abs = num.abs();
                write!(f, "{neg}{radix}{num_abs}{suffix}")
            }
            ExprKind::FieldAccess(expr, ident) => f
                .debug_tuple("FieldAccess")
                .field(&DebugExpr(expr, self.1))
                .field(&ident.symbol)
                .finish(),
            ExprKind::StructLiteral(fields) => {
                let mut f = f.debug_struct("StructLiteral");
                for (ident, field) in fields {
                    f.field(ident.symbol.as_str(), &DebugExpr(field, self.1));
                }
                f.finish()
            }
            ExprKind::String(sym) => {
                write!(f, "{:?}", sym.as_str())
            }
            ExprKind::Match(scrutinee, patterns) => f
                .debug_tuple("Match")
                .field(&DebugExpr(scrutinee, self.1))
                .field(&DebugCases(patterns, self.1))
                .finish(),
            kind => todo!("{kind:#?}"),
        }
    }
}

struct DebugCases<'gcx>(im::Vector<(Pattern, Expr)>, &'gcx GlobalCtxt);

impl fmt::Debug for DebugCases<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(
                self.0
                    .iter()
                    .map(|(p, e)| (DebugPattern(*p, self.1), DebugExpr(*e, self.1))),
            )
            .finish()
    }
}

new_ast_ty!(Item, ItemData, ast, item, kind: ItemKind, span: Span);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemKind {
    Function {
        is_pub: bool,
        name: Ident,
        args: im::Vector<(Ident, Ty)>,
        ret_ty: Option<Ty>,
        add_ret: Span,
        body: Expr,
    },
    TypeDefn {
        name: Ident,
        kind: TypeDefnKind,
    },
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeDefnKind {
    Struct {
        fields: im::Vector<(Ident, Ty)>,
    },
    Sum {
        variants: im::Vector<(Ident, VariantKind)>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VariantKind {
    // TODO: refine tuple syntax & make syntax for tuple structs
    Struct(im::Vector<(Ident, Ty)>),
    Unit,
}

struct DebugItem<'gcx>(Item, &'gcx GlobalCtxt);

impl Item {
    pub fn debug(self, gcx: &'_ GlobalCtxt) -> impl fmt::Debug + '_ {
        DebugItem(self, gcx)
    }
}

impl fmt::Debug for DebugItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.get(self.1).kind {
            ItemKind::Function {
                is_pub,
                name,
                args,
                ret_ty,
                body,
                ..
            } => f
                .debug_struct("Function")
                .field("is_pub", &is_pub)
                .field("name", &name.symbol)
                .field(
                    "args",
                    &args
                        .into_iter()
                        .map(|(id, ty)| (id.symbol, DebugTy(ty, self.1)))
                        .collect::<Vec<_>>(),
                )
                .field("ret_ty", &ret_ty.map(|ret_ty| DebugTy(ret_ty, self.1)))
                .field("body", &DebugExpr(body, self.1))
                .finish(),
            ItemKind::TypeDefn { name, kind } => f
                .debug_struct("TypeDefn")
                .field("name", &name.symbol)
                .field("kind", &DebugTypeDefnKind(kind, self.1))
                .finish(),
            ItemKind::Error => write!(f, "Error"),
        }
    }
}

struct DebugTypeDefnKind<'gcx>(TypeDefnKind, &'gcx GlobalCtxt);

impl TypeDefnKind {
    pub fn debug(self, gcx: &'_ GlobalCtxt) -> impl fmt::Debug + '_ {
        DebugTypeDefnKind(self, gcx)
    }
}

impl fmt::Debug for DebugTypeDefnKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            TypeDefnKind::Struct { fields } => {
                let mut b = f.debug_struct("Struct");
                for &(ident, ty) in fields {
                    b.field(ident.as_str(), &DebugTy(ty, self.1));
                }
                b.finish()
            }
            TypeDefnKind::Sum { variants } => {
                let mut b = f.debug_struct("Sum");
                for (ident, variant) in variants {
                    b.field(ident.as_str(), &DebugVariantKind(variant.clone(), self.1));
                }
                b.finish()
            }
        }
    }
}

struct DebugVariantKind<'gcx>(VariantKind, &'gcx GlobalCtxt);

impl VariantKind {
    pub fn debug(self, gcx: &'_ GlobalCtxt) -> impl fmt::Debug + '_ {
        DebugVariantKind(self, gcx)
    }
}

impl fmt::Debug for DebugVariantKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            VariantKind::Struct(fields) => {
                let mut b = f.debug_struct("Struct");
                for &(ident, ty) in fields {
                    b.field(ident.as_str(), &DebugTy(ty, self.1));
                }
                b.finish()
            }
            VariantKind::Unit => write!(f, "Unit"),
        }
    }
}

new_ast_ty!(Pattern, PatternData, ast, pattern, kind: PatternKind, span: Span);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatternKind {
    StructLike {
        ident: Ident,
        fields: im::Vector<(Ident, Option<Pattern>)>,
        rest: bool,
    },
    TupleLike {
        ident: Ident,
        fields: im::Vector<Pattern>,
        rest: bool,
    },
    Ident(Ident),
    Numeral(i128, Numeral),
    Bool(bool),
    String(Symbol),
    Unit,
    Wildcard,
    Error,
}

struct DebugPattern<'gcx>(Pattern, &'gcx GlobalCtxt);

impl Pattern {
    pub fn debug(self, gcx: &'_ GlobalCtxt) -> impl fmt::Debug + '_ {
        DebugPattern(self, gcx)
    }
}

impl fmt::Debug for DebugPattern<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.get(self.1).kind {
            PatternKind::StructLike {
                ident,
                fields,
                rest,
            } => {
                let mut b = f.debug_struct("StructLike");
                b.field("ident", &ident.symbol);
                for (ident, pat) in fields {
                    b.field(ident.as_str(), &pat.map(|pat| DebugPattern(pat, self.1)));
                }
                if rest {
                    b.finish_non_exhaustive()
                } else {
                    b.finish()
                }
            }
            PatternKind::TupleLike {
                ident,
                fields,
                rest,
            } => {
                let mut b = f.debug_tuple("TupleLike");
                b.field(&ident.symbol);
                for pat in fields {
                    b.field(&DebugPattern(pat, self.1));
                }
                if rest {
                    b.finish_non_exhaustive()
                } else {
                    b.finish()
                }
            }
            PatternKind::Ident(ident) => write!(f, "{}", ident.symbol),
            PatternKind::String(sym) => {
                write!(f, "{:?}", sym.as_str())
            }
            PatternKind::Numeral(num, Numeral::Integer { radix, suffix }) => {
                let neg = if num.is_negative() { "-" } else { "" };
                let num_abs = num.abs();
                write!(f, "{neg}{radix}{num_abs}{suffix}")
            }
            PatternKind::Unit => write!(f, "Unit"),
            PatternKind::Bool(b) => write!(f, "{b:?}"),
            PatternKind::Wildcard => write!(f, "_"),
            PatternKind::Error => write!(f, "Error"),
        }
    }
}

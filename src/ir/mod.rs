use std::{
    cell::{Cell, RefCell},
    collections::{hash_map::Entry, HashMap},
    fmt::{self, Display},
    rc::Rc,
};

use crate::{
    arena::{Arena, IdLike},
    ast::{AstId, BinOpKind, Numeral},
    ctxt::GlobalCtxt,
    parse::{Span, SpanWithFile},
    resolve::PrimTy,
    symbol::{Ident, Symbol},
};

pub const DUMMY_IR_ID: IrId = IrId(0);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IrId(u32);

impl IrId {
    pub fn from_raw(id: u32) -> IrId {
        IrId(id)
    }

    pub fn into_raw(self) -> u32 {
        self.0
    }
}

impl Display for IrId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Item(usize);

impl Item {
    pub fn new(gcx: &GlobalCtxt, id: IrId, kind: ItemKind, span: SpanWithFile) -> Self {
        let x = gcx
            .arenas
            .ir
            .item
            .borrow_mut()
            .push(ItemData { id, kind, span });
        assert_eq!(
            gcx.arenas
                .ir
                .ir_id_to_node
                .borrow_mut()
                .insert(id, Node::Item(x)),
            None
        );
        x
    }
}

impl IdLike for Item {
    fn from_raw(index: usize) -> Self {
        Self(index)
    }

    fn into_raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct ItemData {
    pub id: IrId,
    pub kind: ItemKind,
    pub span: SpanWithFile,
}

#[derive(Clone, Debug)]
pub enum ItemKind {
    Function {
        name: Ident,
        generics: im::Vector<GenericParam>,
        args: im::Vector<(Ident, Ty)>,
        ret_ty: Option<Ty>,
        body: Expr,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Expr(usize);

impl Expr {
    pub fn new(gcx: &GlobalCtxt, id: IrId, kind: ExprKind, span: Span, ty: Ty) -> Expr {
        let expr = gcx
            .arenas
            .ir
            .expr
            .borrow_mut()
            .push(ExprData { id, kind, span });
        assert_eq!(
            gcx.arenas
                .ir
                .ir_id_to_node
                .borrow_mut()
                .insert(id, Node::Expr(expr)),
            None
        );
        gcx.arenas.ir.ty_map.borrow_mut().insert(expr, ty);
        expr
    }
}

impl IdLike for Expr {
    fn from_raw(index: usize) -> Self {
        Self(index)
    }

    fn into_raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct ExprData {
    pub id: IrId,
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum ExprKind {
    Let {
        is_mut: bool,
        name: Ident,
        ty: Option<Ty>,
        val: Expr,
    },
    Closure {
        args: im::Vector<(Ident, Ty)>,
        ret_ty: Option<Ty>,
        body: Expr,
    },
    BinaryOp {
        left: Expr,
        kind: BinOpKind,
        right: Expr,
    },
    UnaryMinus(Expr),
    UnaryNot(Expr),
    Do {
        exprs: im::Vector<Expr>,
    },
    Numeral(Numeral),
    Ident(Ident),
    Bool(bool),
    Unit,
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ty(usize);

impl IdLike for Ty {
    fn from_raw(index: usize) -> Self {
        Self(index)
    }

    fn into_raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct TyData {
    pub id: IrId,
    pub kind: TyKind,
    pub span: Span,
}

impl Ty {
    pub fn new(gcx: &GlobalCtxt, id: IrId, kind: TyKind, span: Span) -> Ty {
        let ty = gcx
            .arenas
            .ir
            .ty
            .borrow_mut()
            .push(TyData { id, kind, span });
        assert_eq!(
            gcx.arenas
                .ir
                .ir_id_to_node
                .borrow_mut()
                .insert(id, Node::Ty(ty)),
            None
        );
        ty
    }

    pub fn kind(self, gcx: &GlobalCtxt) -> Kind {
        match gcx.arenas.ir.ty(self).kind {
            TyKind::Primitive(_) | TyKind::Unit => Kind::Monotype,
            TyKind::Function(ins, out) => {
                #[cfg(debug_assertions)]
                {
                    for ty in ins {
                        assert_eq!(ty.kind(gcx), Kind::Monotype);
                    }
                    if let Some(out) = out {
                        assert_eq!(out.kind(gcx), Kind::Monotype);
                    }
                }
                Kind::Monotype
            }
            TyKind::PolyFunction(_, ins, out) => {
                #[cfg(debug_assertions)]
                {
                    for ty in ins {
                        assert_eq!(ty.kind(gcx), Kind::Monotype);
                    }
                    if let Some(out) = out {
                        assert_eq!(out.kind(gcx), Kind::Monotype);
                    }
                }
                Kind::Polytype
            }
            TyKind::Meta(_, spine) => {
                #[cfg(debug_assertions)]
                {
                    for ty in spine {
                        assert_eq!(ty.kind(gcx), Kind::Monotype);
                    }
                }
                Kind::Monotype
            }
            TyKind::Free(_) => Kind::Monotype,
            TyKind::Var(_) => Kind::Monotype,
        }
    }

    pub fn force(self, gcx: &GlobalCtxt) -> Ty {
        match gcx.arenas.ir.ty(self).kind {
            TyKind::Meta(mv, spine) => match mv.clone().0.borrow().0 {
                MetaEntry::Solved(t) => {
                    // TODO: is this valid?
                    t.instantiate(gcx, &spine.into_iter().collect::<Vec<_>>())
                        .force(gcx)
                }
                MetaEntry::Unsolved => self,
            },
            _ => self,
        }
    }

    pub fn instantiate(self, gcx: &GlobalCtxt, env: &[Ty]) -> Ty {
        fn inner(this: Ty, gcx: &GlobalCtxt, env: &HashMap<IrId, Ty>) -> Ty {
            match gcx.arenas.ir.ty(this).kind {
                TyKind::Function(args, ret_ty) => {
                    let args = args.into_iter().map(|arg| inner(arg, gcx, env)).collect();
                    let ret_ty = ret_ty.map(|ret_ty| inner(ret_ty, gcx, env));
                    Ty::new(
                        gcx,
                        gcx.arenas.ir.next_id(),
                        TyKind::Function(args, ret_ty),
                        gcx.arenas.ir.ty(this).span,
                    )
                }
                TyKind::PolyFunction(params, args, ret_ty) => {
                    let args = args.into_iter().map(|arg| inner(arg, gcx, env)).collect();
                    let ret_ty = ret_ty.map(|ret_ty| inner(ret_ty, gcx, env));
                    Ty::new(
                        gcx,
                        gcx.arenas.ir.next_id(),
                        TyKind::PolyFunction(params, args, ret_ty),
                        gcx.arenas.ir.ty(this).span,
                    )
                }
                TyKind::Meta(mv, spine) => {
                    let spine = spine.into_iter().map(|ty| inner(ty, gcx, env)).collect();
                    Ty::new(
                        gcx,
                        gcx.arenas.ir.next_id(),
                        TyKind::Meta(mv, spine),
                        gcx.arenas.ir.ty(this).span,
                    )
                }
                TyKind::Var(id) => {
                    if let Some(ty) = env.get(&id) {
                        *ty
                    } else {
                        this
                    }
                }
                _ => this,
            }
        }

        let TyKind::PolyFunction(params, args, ret_ty) = gcx.arenas.ir.ty(self).kind else {
            return self;
        };

        let env = env
            .iter()
            .zip(params)
            .map(|(ty, param)| (param.id, *ty))
            .collect();

        let args = args.into_iter().map(|arg| inner(arg, gcx, &env)).collect();
        let ret_ty = ret_ty.map(|ret_ty| inner(ret_ty, gcx, &env));
        Ty::new(
            gcx,
            gcx.arenas.ir.next_id(),
            TyKind::Function(args, ret_ty),
            gcx.arenas.ir.ty(self).span,
        )
    }
}

#[derive(Clone, Debug)]
pub enum TyKind {
    Function(im::Vector<Ty>, Option<Ty>),
    PolyFunction(im::Vector<GenericParam>, im::Vector<Ty>, Option<Ty>),
    Meta(MetaVar, im::Vector<Ty>),
    Free(IrId),
    Var(IrId),
    Primitive(PrimTy),
    Unit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenericParam {
    pub id: IrId,
    pub ident: Ident,
}

impl GenericParam {
    pub fn new(gcx: &GlobalCtxt, id: IrId, ident: Ident) -> Self {
        gcx.arenas
            .ir
            .ir_id_to_node
            .borrow_mut()
            .insert(id, Node::GenericParam(GenericParam { id, ident }));
        GenericParam { id, ident }
    }
}

#[derive(Debug)]
pub struct IrArenas {
    pub expr: RefCell<Arena<Expr, ExprData>>,
    pub ty: RefCell<Arena<Ty, TyData>>,
    pub item: RefCell<Arena<Item, ItemData>>,
    next_ir_id: Cell<u32>,
    ast_id_to_ir_id: RefCell<HashMap<AstId, IrId>>,
    ir_id_to_node: RefCell<HashMap<IrId, Node>>,
    ty_map: RefCell<HashMap<Expr, Ty>>,
}

impl IrArenas {
    pub fn clear(&self) {
        self.next_ir_id.set(1);
        self.ast_id_to_ir_id.borrow_mut().clear();
    }

    pub fn expr(&self, id: Expr) -> ExprData {
        self.expr.borrow()[id].clone()
    }

    pub fn ty(&self, id: Ty) -> TyData {
        self.ty.borrow()[id].clone()
    }

    pub fn item(&self, id: Item) -> ItemData {
        self.item.borrow()[id].clone()
    }

    pub fn next_id(&self) -> IrId {
        let id = self.next_ir_id.get();
        assert!(id < u32::MAX);
        self.next_ir_id.set(id + 1);
        IrId::from_raw(id)
    }

    pub fn lower_id(&self, id: AstId) -> IrId {
        match self.ast_id_to_ir_id.borrow_mut().entry(id) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let id = self.next_id();
                entry.insert(id);
                id
            }
        }
    }

    pub fn get_node_by_id(&self, id: IrId) -> Option<Node> {
        self.ir_id_to_node.borrow().get(&id).cloned()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    Expr(Expr),
    Ty(Ty),
    Item(Item),
    GenericParam(GenericParam),
}

impl Default for IrArenas {
    fn default() -> Self {
        Self {
            expr: Default::default(),
            ty: Default::default(),
            item: Default::default(),
            next_ir_id: Cell::new(1),
            ast_id_to_ir_id: Default::default(),
            ir_id_to_node: Default::default(),
            ty_map: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Monotype,
    Polytype,
}

#[derive(Clone, Debug)]
pub struct MetaVar(pub Rc<RefCell<(MetaEntry, MetaInfo)>>);

#[derive(Debug)]
pub enum MetaEntry {
    Solved(Ty),
    Unsolved,
}

#[derive(Debug)]
pub struct MetaInfo {
    pub name: Symbol,
    pub span: Span,
}

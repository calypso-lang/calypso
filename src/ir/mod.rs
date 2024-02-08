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
}

#[derive(Clone, Debug)]
pub enum TyKind {
    Function(im::Vector<Ty>, Option<Ty>),
    PolyFunction(im::Vector<GenericParam>, im::Vector<Ty>, Option<Ty>),
    Meta(MetaVar, im::Vector<Ty>),
    InsertedMeta(MetaVar),
    Free(IrId),
    Var(IrId),
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

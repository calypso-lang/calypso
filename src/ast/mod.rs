pub mod visitor;

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt::{self, Display},
};

use crate::{
    arena::{Arena, IdLike},
    ctxt::GlobalCtxt,
    parse::{Span, SpanWithFile},
    resolve::ResolutionData,
    symbol::{Ident, Symbol},
};

pub const DUMMY_AST_ID: AstId = AstId(0);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AstId(u32);

impl AstId {
    pub fn from_raw(id: u32) -> AstId {
        AstId(id)
    }

    pub fn into_raw(self) -> u32 {
        self.0
    }
}

impl Display for AstId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Item(usize);

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
    pub id: AstId,
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

impl Item {
    pub fn new(gcx: &GlobalCtxt, kind: ItemKind, span: SpanWithFile) -> Item {
        let id = gcx.arenas.ast.next_ast_id();
        let item = gcx
            .arenas
            .ast
            .item
            .borrow_mut()
            .push(ItemData { id, kind, span });
        gcx.arenas.ast.insert_node(id, Node::Item(item));
        item
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Expr(usize);

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
    pub id: AstId,
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(gcx: &GlobalCtxt, kind: ExprKind, span: Span) -> Expr {
        let id = gcx.arenas.ast.next_ast_id();
        let expr = gcx
            .arenas
            .ast
            .expr
            .borrow_mut()
            .push(ExprData { id, kind, span });
        gcx.arenas.ast.insert_node(id, Node::Expr(expr));
        expr
    }
}

#[derive(Clone, Debug)]
pub enum ExprKind {
    ItemStmt(Item),
    Let {
        is_mut: bool,
        name: Ident,
        ty: Option<Ty>,
        val: Expr,
    },
    Closure {
        args: im::Vector<(Ident, Option<Ty>)>,
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
    Call(Expr, im::Vector<Expr>),
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
    pub id: AstId,
    pub kind: TyKind,
    pub span: Span,
}

impl Ty {
    pub fn new(gcx: &GlobalCtxt, kind: TyKind, span: Span) -> Ty {
        let id = gcx.arenas.ast.next_ast_id();
        let ty = gcx
            .arenas
            .ast
            .ty
            .borrow_mut()
            .push(TyData { id, kind, span });
        gcx.arenas.ast.insert_node(id, Node::Ty(ty));
        ty
    }
}

#[derive(Clone, Debug)]
pub enum TyKind {
    Function(im::Vector<Ty>, Option<Ty>),
    Ident(Ident),
}

#[derive(Clone, Debug)]
pub struct GenericParam {
    pub id: AstId,
    pub ident: Ident,
}

impl GenericParam {
    pub fn new(gcx: &GlobalCtxt, ident: Ident) -> GenericParam {
        let id = gcx.arenas.ast.next_ast_id();
        let param = GenericParam { id, ident };
        gcx.arenas
            .ast
            .ast_id_to_node
            .borrow_mut()
            .insert(id, Node::GenericParam(param.clone()));
        param
    }
}

#[derive(Copy, Clone, Debug)]
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
}

#[derive(Debug)]
pub struct AstArenas {
    pub expr: RefCell<Arena<Expr, ExprData>>,
    pub ty: RefCell<Arena<Ty, TyData>>,
    pub item: RefCell<Arena<Item, ItemData>>,
    next_ast_id: Cell<u32>,
    ast_id_to_node: RefCell<HashMap<AstId, Node>>,
    pub(crate) res_data: RefCell<ResolutionData>,
}

impl AstArenas {
    pub fn clear(&self) {
        self.next_ast_id.set(1);
        self.ast_id_to_node.borrow_mut().clear();
        self.res_data.borrow_mut().clear();
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

    pub fn next_ast_id(&self) -> AstId {
        let id = self.next_ast_id.get();
        assert!(id < u32::MAX);
        self.next_ast_id.set(id + 1);
        AstId::from_raw(id)
    }

    pub fn get_node_by_id(&self, id: AstId) -> Option<Node> {
        self.ast_id_to_node.borrow().get(&id).cloned()
    }

    pub fn into_iter_nodes(&self) -> impl Iterator<Item = Node> {
        let v = self.ast_id_to_node.borrow();
        v.values().cloned().collect::<Vec<_>>().into_iter()
    }

    fn insert_node(&self, id: AstId, node: Node) {
        self.ast_id_to_node.borrow_mut().insert(id, node);
    }
}

impl Default for AstArenas {
    fn default() -> Self {
        Self {
            expr: Default::default(),
            ty: Default::default(),
            item: Default::default(),
            next_ast_id: Cell::new(1),
            ast_id_to_node: Default::default(),
            res_data: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Node {
    Expr(Expr),
    Ty(Ty),
    Item(Item),
    GenericParam(GenericParam),
}

impl Node {
    pub fn ident(self, gcx: &GlobalCtxt) -> Option<Ident> {
        match self {
            Node::Expr(e) => match gcx.arenas.ast.expr(e).kind {
                ExprKind::Let { name, .. } => Some(name),
                _ => None,
            },
            Node::Ty(t) => match gcx.arenas.ast.ty(t).kind {
                TyKind::Ident(id) => Some(id),
                _ => None,
            },
            Node::Item(i) => match gcx.arenas.ast.item(i).kind {
                ItemKind::Function { name, .. } => Some(name),
            },
            Node::GenericParam(param) => Some(param.ident),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Number radixes.
pub enum Radix {
    /// No prefix (`0d` by default)
    None,
    /// `0d`
    Decimal,
    /// `0b`
    Binary,
    /// `0o`
    Octal,
    /// `0x`
    Hexadecimal,
}

impl Radix {
    #[must_use]
    pub fn radix(self) -> u32 {
        match self {
            Self::None | Self::Decimal => 10,
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Hexadecimal => 16,
        }
    }
}

impl Display for Radix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decimal => write!(f, "0d"),
            Self::Binary => write!(f, "0b"),
            Self::Octal => write!(f, "0o"),
            Self::Hexadecimal => write!(f, "0x"),
            Self::None => Ok(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Number suffixes.
pub enum Suffix {
    /// `u`
    Uint,
    /// `s`
    Sint,
}

impl Display for Suffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uint => write!(f, "u"),
            Self::Sint => write!(f, "s"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Numeral {
    Integer {
        suffix: Option<Suffix>,
        radix: Radix,
        sym: Symbol,
    },
    Float {
        from_integer: bool,
        sym: Symbol,
    },
}

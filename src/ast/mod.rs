use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt::{self, Display},
};

use id_arena::{Arena, Id};

use crate::{
    ctxt::GlobalCtxt,
    parse::Span,
    symbol::{Ident, Symbol},
};

pub const DUMMY_AST_ID: AstId = AstId { _raw: 0 };

index_vec::define_index_type! {
    pub struct AstId = u32;

    DISABLE_MAX_INDEX_CHECK = cfg!(not(debug_assertions));
    DEBUG_FORMAT = "AstId({})";
    DISPLAY_FORMAT = "{}";
    IMPL_RAW_CONVERSIONS = true;
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub id: AstId,
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(gcx: &GlobalCtxt, kind: ExprKind, span: Span) -> Id<Expr> {
        let id = gcx.arenas.ast.next_ast_id();
        let expr = gcx
            .arenas
            .ast
            .expr
            .borrow_mut()
            .alloc(Expr { id, kind, span });
        gcx.arenas.ast.insert_node(id, Node::Expr(expr));
        expr
    }
}

#[derive(Clone, Debug)]
pub enum ExprKind {
    Let {
        is_mut: bool,
        varlist: im::Vector<(Ident, Option<Id<Ty>>, Id<Expr>)>,
        in_block: im::Vector<Id<Expr>>,
    },
    BinaryOp {
        left: Id<Expr>,
        kind: BinOpKind,
        right: Id<Expr>,
    },
    UnaryMinus(Id<Expr>),
    UnaryNot(Id<Expr>),
    //Paren(Id<Expr>),
    Do {
        exprs: im::Vector<Id<Expr>>,
    },
    Numeral(Numeral),
    Ident(Ident),
    Bool(bool),
    Error,
}

#[derive(Clone, Debug)]
pub struct Ty {
    pub id: AstId,
    pub kind: TyKind,
    pub span: Span,
}

impl Ty {
    pub fn new(gcx: &GlobalCtxt, kind: TyKind, span: Span) -> Id<Ty> {
        let id = gcx.arenas.ast.next_ast_id();
        let ty = gcx.arenas.ast.ty.borrow_mut().alloc(Ty { id, kind, span });
        gcx.arenas.ast.insert_node(id, Node::Ty(ty));
        ty
    }
}

#[derive(Clone, Debug)]
pub enum TyKind {
    Primitive(Primitive),
}

#[derive(Copy, Clone, Debug)]
pub enum Primitive {
    Bool,
    Uint,
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

#[derive(Debug, Default)]
pub struct Parentage {
    pub map: HashMap<AstId, AstId>,
}

#[derive(Debug)]
pub struct AstArenas {
    pub expr: RefCell<Arena<Expr>>,
    pub ty: RefCell<Arena<Ty>>,
    pub parentage: RefCell<Parentage>,
    next_ast_id: Cell<u32>,
    ast_id_to_node: RefCell<HashMap<AstId, Node>>,
}

impl AstArenas {
    pub fn clear(&self) {
        self.next_ast_id.set(1);
        self.ast_id_to_node.borrow_mut().clear();
        self.parentage.borrow_mut().map.clear();
    }

    pub fn expr(&self, id: Id<Expr>) -> Expr {
        self.expr.borrow()[id].clone()
    }

    pub fn ty(&self, id: Id<Ty>) -> Ty {
        self.ty.borrow()[id].clone()
    }

    pub fn next_ast_id(&self) -> AstId {
        let id = self.next_ast_id.get();
        assert!(id < u32::MAX);
        self.next_ast_id.set(id + 1);
        AstId::from_raw(id)
    }

    pub fn get_node_by_id(&self, id: AstId) -> Option<Node> {
        self.ast_id_to_node.borrow().get(&id).copied()
    }

    pub fn into_iter_nodes(&self) -> impl Iterator<Item = Node> {
        let v = self.ast_id_to_node.borrow();
        v.values().copied().collect::<Vec<_>>().into_iter()
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
            parentage: Default::default(),
            next_ast_id: Cell::new(1),
            ast_id_to_node: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Node {
    Expr(Id<Expr>),
    Ty(Id<Ty>),
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

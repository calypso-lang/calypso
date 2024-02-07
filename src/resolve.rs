use std::collections::HashMap;

use ariadne::{Color, Config, Label, LabelAttach, ReportKind};
use chumsky::container::Seq;

use crate::{
    ast::{
        AstArenas, AstId, Expr, ExprKind, Item, ItemData, ItemKind, Node, Ty, TyKind, DUMMY_AST_ID,
    },
    ctxt::GlobalCtxt,
    diagnostic::Diagnostic,
    error::CalResult,
    parse::{Span, SpanWithFile},
    symbol::{Ident, Symbol},
};

/// Resolved name mappings generated during a resolution pass.
///
/// As multiple [`AstId`]s can map to a single resolved name, such as in `\x.f x
/// x`, where each `x` has a different id, but refers to the same definition,
/// this data structure uses a vector and map-to-vector-index to prevent wasting
/// space--in the future, [`Res`]olution data will likely be slightly more
/// expensive to store (due to module paths and whatnot, which will likely be
/// added at some point in the future).
///
/// Note that only ids referring to [`ExprKind::Name`] or [`TyKind::Name`] are
/// assigned resolution data.
#[derive(Debug, Default)]
pub struct ResolutionData {
    ast_id_to_res_idx: HashMap<AstId, usize>,
    res_data: Vec<Res>,
}

impl ResolutionData {
    pub fn clear(&mut self) {
        self.ast_id_to_res_idx.clear();
        self.res_data.clear();
    }

    pub(crate) fn insert(&mut self, id: AstId, res: Res) {
        let idx = self
            .res_data
            .iter()
            .enumerate()
            .find_map(|(idx, res1)| (res1 == &res).then_some(idx))
            .unwrap_or_else(|| {
                let idx = self.res_data.len();
                self.res_data.push(res);
                idx
            });
        self.ast_id_to_res_idx.insert(id, idx);
    }

    pub fn get_by_id(&'_ self, id: AstId) -> Option<&'_ Res> {
        self.ast_id_to_res_idx
            .get(&id)
            .and_then(|idx| self.res_data.get(*idx))
    }

    pub fn to_hash_map(&'_ self) -> HashMap<AstId, &'_ Res> {
        self.ast_id_to_res_idx
            .iter()
            .flat_map(|(&id, &idx)| Some((id, self.res_data.get(idx)?)))
            .collect()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Res {
    /// A primitive type, e.g. `Int`.
    ///
    /// **Belongs to the type namespace.**
    PrimTy(PrimTy),
    /// A primitive function, e.g. `add`
    PrimFunc(PrimFunc),
    /// Corresponds to something defined in user code, with a unique
    /// [`AstId`].
    ///
    /// **Does not belong to a specific namespace.**
    Defn(DefnKind, AstId),
    /// A local variable or function parameter.
    ///
    /// **Belongs to the value namespace.**
    Local(AstId),
    /// A dummy [`Res`] variant representing a resolution error, so
    /// compilation can continue to gather further errors before
    /// crashing.
    ///
    /// **Does not belong to a specific namespace.**
    Err,
}

impl Res {
    pub fn id(self) -> Option<AstId> {
        match self {
            Res::PrimTy(_) | Res::Err | Res::PrimFunc(_) => None,
            Res::Defn(_, id) | Res::Local(id) => Some(id),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimTy {
    Int,
    Uint,
    Boolean,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimFunc {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DefnKind {
    Fn,
    TyAlias,
    Enum,
    /// Refers to an enum variant. See [`DefKind::Ctor`] for its
    /// constructor.
    Variant,
    Ctor(CtorOf, CtorKind),
    /// Refers to the struct itself. See [`DefKind::Ctor`] for its
    /// constructor.
    Struct,
    Primitive,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CtorOf {
    Struct,
    Variant,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CtorKind {
    Fn,
    Const,
}

pub fn resolve_code_unit(gcx: &GlobalCtxt, items: &[Item]) -> CalResult<()> {
    let arena = &gcx.arenas.ast;
    let mut rcx = ResolutionCtxt {
        gcx,
        arena,
        global_value_ns: HashMap::new(),
        global_type_ns: HashMap::new(),
        defn_id_to_span: HashMap::new(),
        ty_scope_stack: vec![],
        value_scope_stack: vec![],
    };
    rcx.global_type_ns.insert(
        Symbol::intern_static("UInt"),
        (DUMMY_AST_ID, DefnKind::Primitive),
    );
    rcx.global_type_ns.insert(
        Symbol::intern_static("Int"),
        (DUMMY_AST_ID, DefnKind::Primitive),
    );
    rcx.global_type_ns.insert(
        Symbol::intern_static("Bool"),
        (DUMMY_AST_ID, DefnKind::Primitive),
    );
    rcx.collect(items)?;
    for item in items {
        rcx.resolve_item(*item)?;
    }
    Ok(())
}

struct ResolutionCtxt<'gcx> {
    gcx: &'gcx GlobalCtxt,
    arena: &'gcx AstArenas,
    global_value_ns: HashMap<Symbol, (AstId, DefnKind)>,
    global_type_ns: HashMap<Symbol, (AstId, DefnKind)>,
    defn_id_to_span: HashMap<AstId, SpanWithFile>,
    ty_scope_stack: Vec<Scope>,
    value_scope_stack: Vec<Scope>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Scope {
    bindings: HashMap<Symbol, Res>,
    kind: ScopeKind,
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Self {
        Scope {
            bindings: HashMap::new(),
            kind,
        }
    }
}

/// Types of scopes passed through, and their restrictions.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ScopeKind {
    /// No restriction is applied.
    Normal,
    /// We passed through an item scope. Disallow upvars.
    Item,
}

impl<'gcx> ResolutionCtxt<'gcx> {
    fn report_duplicate_name(
        &self,
        _item: ItemData,
        ident: Ident,
        kind: DefnKind,
        duplicate: AstId,
        dup_kind: DefnKind,
    ) {
        let ident_span = // if let DefnKind::EnumConstructor(branch) = dup_kind {
        //     let Node::Item(i) = self.arena.get_node_by_id(duplicate).unwrap() else {
        //         unreachable!()
        //     };
        //     let ItemKind::Enum(_, cons, _) = self.arena.item(i).kind else {
        //         unreachable!()
        //     };
        //     cons.get(branch).unwrap().0.span
        // } else
	    // if let DefnKind::Generic(ix) = dup_kind {
        //     let Node::Item(i) = self.arena.get_node_by_id(duplicate).unwrap() else {
        //         unreachable!()
        //     };
        //     let ItemKind::Enum(generics, _, _) = self.arena.item(i).kind else {
        //         unreachable!()
        //     };
        //     generics.get(ix).unwrap().span
        // } else
	    if let DefnKind::Primitive = dup_kind {
            // Primitives have a dummy AST ID which will fail the
            // expect below, so we special case them here.
            (u32::MAX..u32::MAX).into()
        } else {
            self.arena
                .get_node_by_id(duplicate)
                    .expect("defn_id in nodes")
                .ident(self.gcx)
                .unwrap()
                .span
        };

        let span: Span = ident_span;
        let file = self.gcx.session.borrow().current_file.unwrap();
        let report = Diagnostic::build(
            ReportKind::Error,
            Symbol::intern_static("<unknown>"),
            span.lo() as usize,
        )
        .with_message(format!(
            "the name `{}` is defined multiple times",
            ident.symbol
        ))
        .with_label(
            Label::new((file, span).into())
                .with_message("first defined here")
                .with_color(Color::Blue),
        )
        .with_label(
            Label::new((file, ident.span).into())
                .with_message("redefined here")
                .with_color(Color::Red),
        )
        .with_note(match (kind, dup_kind) {
            (DefnKind::Fn, _) => "top-level `fn`s must have unique names",

            _ => unreachable!(),
        })
        .with_config(Config::default().with_label_attach(LabelAttach::End))
        .finish();

        let mut drcx = self.gcx.diag.borrow_mut();
        drcx.report_syncd(report);
    }

    /// Collect all item names.
    fn collect(&mut self, items: &[Item]) -> CalResult<()> {
        for item in items {
            let item = self.arena.item(*item);
            match item.kind {
                ItemKind::Function { name, .. } => {
                    if let Some(&(defn_id, defn_kind)) = self.global_value_ns.get(&name) {
                        self.report_duplicate_name(
                            item.clone(),
                            name,
                            DefnKind::Fn,
                            defn_id,
                            defn_kind,
                        );
                    } else {
                        self.global_value_ns
                            .insert(name.symbol, (item.id, DefnKind::Fn));
                    }
                    self.defn_id_to_span.insert(item.id, item.span);
                }
            }
        }
        Ok(())
    }

    fn resolve_item(&mut self, item: Item) -> CalResult<()> {
        let item = self.arena.item(item);
        match item.kind {
            ItemKind::Function {
                name: _,
                args,
                ret_ty,
                body,
            } => {
                let mut scope = Scope::new(ScopeKind::Item);
                for (arg, ty) in args {
                    self.resolve_ty(ty)?;
                    scope.bindings.insert(arg.symbol, Res::Local(item.id));
                }
                self.value_scope_stack.push(scope);
                if let Some(ret_ty) = ret_ty {
                    self.resolve_ty(ret_ty)?;
                }
                self.resolve_expr(body)?;
                self.value_scope_stack.pop();
            }
        }
        Ok(())
    }

    fn find_ty_in_scope(&self, name: Symbol) -> Option<Res> {
        for val in self.ty_scope_stack.iter().rev() {
            if let Some(res) = val.bindings.get(&name) {
                return Some(*res);
            }
        }
        None
    }

    fn find_expr_in_scope(&self, name: Symbol) -> Option<Res> {
        let mut continue_next = true;
        for val in self.value_scope_stack.iter().rev() {
            if !continue_next {
                break;
            }
            if val.kind == ScopeKind::Item {
                continue_next = false;
            }

            if let Some(res) = val.bindings.get(&name) {
                return Some(*res);
            }
        }
        None
    }

    fn resolve_ty(&mut self, ty: Ty) -> CalResult<()> {
        let ty = self.arena.ty(ty);
        match ty.kind {
            TyKind::Function(args, ret_ty) => {
                for ty in args {
                    self.resolve_ty(ty)?;
                }
                if let Some(ret_ty) = ret_ty {
                    self.resolve_ty(ret_ty)?;
                }
            }
            TyKind::Ident(name) => {
                let res = if let Some(res) = self.find_ty_in_scope(*name) {
                    res
                } else if let Some(&(defn, defn_kind)) = self.global_type_ns.get(&name) {
                    Res::Defn(defn_kind, defn)
                } else if name.as_str() == "_" {
                    Res::Err
                } else {
                    let file = self.gcx.session.borrow().current_file.unwrap();
                    let report =
                        Diagnostic::build(ReportKind::Error, file, name.span.lo() as usize)
                            .with_message(format!(
                                "cannot find type `{}` in this scope",
                                name.symbol
                            ))
                            .with_label(
                                Label::new((file, name.span).into())
                                    .with_message("not found in this scope")
                                    .with_color(Color::Red),
                            )
                            .finish();
                    let mut drcx = self.gcx.diag.borrow_mut();
                    drcx.report_syncd(report);
                    drop(drcx);
                    Res::Err
                };
                self.arena.res_data.borrow_mut().insert(ty.id, res);
            }
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: Expr) -> CalResult<()> {
        let expr = self.arena.expr(expr);
        match expr.kind {
            ExprKind::Let { name, ty, val, .. } => {
                let mut scope = Scope::new(ScopeKind::Normal);
                scope.bindings.insert(*name, Res::Local(expr.id));
                self.value_scope_stack.push(scope);
                if let Some(ty) = ty {
                    self.resolve_ty(ty)?;
                }
                self.resolve_expr(val)?;
            }
            ExprKind::BinaryOp { left, right, .. } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            ExprKind::UnaryMinus(expr) | ExprKind::UnaryNot(expr) => {
                self.resolve_expr(expr)?;
            }
            ExprKind::Do { exprs } => {
                let scope_len = self.value_scope_stack.len();
                for expr in exprs {
                    self.resolve_expr(expr)?;
                }
                self.value_scope_stack.truncate(scope_len);
            }
            ExprKind::Ident(name) => {
                let res = if let Some(res) = self.find_expr_in_scope(*name) {
                    res
                } else if let Some(&(defn, defn_kind)) = self.global_value_ns.get(&name) {
                    Res::Defn(defn_kind, defn)
                } else if name.as_str() == "_" {
                    Res::Err
                } else {
                    let file = self.gcx.session.borrow().current_file.unwrap();
                    let report =
                        Diagnostic::build(ReportKind::Error, file, name.span.lo() as usize)
                            .with_message(format!(
                                "cannot find value `{}` in this scope",
                                name.symbol
                            ))
                            .with_label(
                                Label::new((file, name.span).into())
                                    .with_message("not found in this scope")
                                    .with_color(Color::Red),
                            )
                            .finish();
                    let mut drcx = self.gcx.diag.borrow_mut();
                    drcx.report_syncd(report);
                    drop(drcx);
                    Res::Err
                };
                self.arena.res_data.borrow_mut().insert(expr.id, res);
            }
            ExprKind::Bool(_) | ExprKind::Numeral(_) | ExprKind::Error => {}
        }
        Ok(())
    }
}

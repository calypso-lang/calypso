use std::rc::Rc;

use crate::{
    ctxt::GlobalCtxt,
    ir::{GenericParam, IrId, MetaEntry, MetaVar, Node, Ty, TyKind},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnifyError {
    Occurs,
    Scope(GenericParam),
    SpineMismatch,
    RigidMismatch,
}

fn unify_spine(
    gcx: &GlobalCtxt,
    sp1: im::Vector<Ty>,
    sp2: im::Vector<Ty>,
) -> Result<(), UnifyError> {
    if sp1.len() != sp2.len() {
        return Err(UnifyError::SpineMismatch);
    }
    for (t1, t2) in sp1.into_iter().zip(sp2.into_iter()) {
        unify(gcx, t1, t2)?;
    }
    Ok(())
}

fn rename_spine(
    gcx: &GlobalCtxt,
    m: MetaVar,
    env: im::Vector<GenericParam>,
    sp: im::Vector<Ty>,
) -> Result<im::Vector<Ty>, UnifyError> {
    sp.into_iter()
        .map(move |a| rename(gcx, m.clone(), env.clone(), a))
        .collect()
}

fn rename(
    gcx: &GlobalCtxt,
    m: MetaVar,
    mut env: im::Vector<GenericParam>,
    t: Ty,
) -> Result<Ty, UnifyError> {
    use TyKind::*;
    let t = t.force(gcx);
    let tdat = gcx.arenas.ir.ty(t);
    Ok(match tdat.kind {
        Unit => t,
        Meta(m1, _) if Rc::ptr_eq(&m1.0, &m.0) => return Err(UnifyError::Occurs),
        Meta(m1, sp) => Ty::new(
            gcx,
            gcx.arenas.ir.next_id(),
            Meta(m1, rename_spine(gcx, m, env, sp)?),
            tdat.span,
        ),
        Var(id) => match env.iter().find(|x| x.id == id) {
            None => {
                return Err(UnifyError::Scope({
                    let Node::GenericParam(param) = gcx.arenas.ir.get_node_by_id(id).unwrap()
                    else {
                        unreachable!()
                    };
                    param
                }))
            }
            Some(_) => t,
        },
        Function(args, ret_ty) => {
            let args = rename_spine(gcx, m.clone(), env.clone(), args)?;
            let ret_ty = ret_ty
                .map(|ret_ty| rename(gcx, m, env, ret_ty))
                .transpose()?;
            Ty::new(
                gcx,
                gcx.arenas.ir.next_id(),
                Function(args, ret_ty),
                tdat.span,
            )
        }
        PolyFunction(params, args, ret_ty) => {
            env.extend(params.clone());
            let args = rename_spine(gcx, m.clone(), env.clone(), args)?;
            let ret_ty = ret_ty
                .map(|ret_ty| rename(gcx, m, env, ret_ty))
                .transpose()?;
            Ty::new(
                gcx,
                gcx.arenas.ir.next_id(),
                PolyFunction(params, args, ret_ty),
                tdat.span,
            )
        }
        Free(_) => t,
        Primitive(_) => t,
    })
}

fn solve(gcx: &GlobalCtxt, m: MetaVar, sp: im::Vector<Ty>, rhs: Ty) -> Result<(), UnifyError> {
    let env = sp
        .into_iter()
        .map(|x| {
            let TyKind::Var(id) = gcx.arenas.ir.ty(x).kind else {
                unreachable!()
            };
            let Node::GenericParam(param) = gcx.arenas.ir.get_node_by_id(id).unwrap() else {
                unreachable!()
            };
            param
        })
        .collect::<im::Vector<_>>();

    let sol = rename(gcx, m.clone(), env, rhs)?;
    m.0.borrow_mut().0 = MetaEntry::Solved(sol);
    Ok(())
}

pub fn unify(gcx: &GlobalCtxt, t: Ty, u: Ty) -> Result<(), UnifyError> {
    use TyKind::*;

    let t = t.force(gcx);
    let u = u.force(gcx);
    let tdat = gcx.arenas.ir.ty(t);
    let udat = gcx.arenas.ir.ty(u);

    match (tdat.kind.clone(), udat.kind.clone()) {
        (Meta(m1, sp1), Meta(m2, sp2)) if Rc::ptr_eq(&m1.0, &m2.0) => {
            unify_spine(gcx, sp1, sp2)?;
        }
        (Meta(m1, sp1), _) => solve(gcx, m1, sp1, u)?,
        (_, Meta(m2, sp2)) => solve(gcx, m2, sp2, t)?,
        (Var(id1), Var(id2)) if id1 == id2 => {}
        (Function(args1, ret_ty1), Function(args2, ret_ty2)) => {
            if (ret_ty1.is_some() && ret_ty2.is_none()) || (ret_ty1.is_none() && ret_ty2.is_some())
            {
                return Err(UnifyError::RigidMismatch);
            }

            unify_spine(gcx, args1, args2)?;
            if let Some((ret_ty1, ret_ty2)) = ret_ty1.zip(ret_ty2) {
                unify(gcx, ret_ty1, ret_ty2)?;
            }
        }
        (Free(n1), Free(n2)) if n1 == n2 => {}
        (Primitive(p1), Primitive(p2)) if p1 == p2 => {}
        (Unit, Unit) => {}
        _ => return Err(UnifyError::RigidMismatch),
    }

    Ok(())
}

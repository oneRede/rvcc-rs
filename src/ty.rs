use crate::{
    rvcc::{
        NodeKind, NodeWrap, TypeKind, TyWrap,
    },
    utils::error_token,
};

#[allow(dead_code)]
pub fn is_int(ty: TyWrap) -> bool {
    return ty.kind() == Some(TypeKind::INT);
}

#[allow(dead_code)]
pub fn add_ty(node: NodeWrap) {
    if node.ptr.is_none() || !node.ty().ptr.is_none() {
        return;
    }

    add_ty(node.lhs());
    add_ty(node.rhs());
    add_ty(node.cond());
    add_ty(node.then());
    add_ty(node.els());
    add_ty(node.init());
    add_ty(node.inc());

    let mut next = node.body();
    while !next.ptr.is_none() {
        add_ty(next);
        next = next.next();
    }

    let mut next = node.args();
    while !next.ptr.is_none() {
        add_ty(next);
        next = next.next();
    }

    match node.kind() {
        NodeKind::NEG | NodeKind::Div | NodeKind::Mul | NodeKind::Sub | NodeKind::Add => {
            node.set_ty(node.lhs().ty());
            return;
        }
        NodeKind::ASSIGN => {
            let kind = node.lhs().ty().kind();
            if kind == Some(TypeKind::ARRAY) {
                error_token(node.lhs().token(), "not an lvalue");
            }
            node.set_ty(node.lhs().ty());
            return;
        }
        NodeKind::FUNCALL
        | NodeKind::EQ
        | NodeKind::NE
        | NodeKind::LT
        | NodeKind::LE
        | NodeKind::Num => {
            node.set_ty(TyWrap::new_with_kind(Some(TypeKind::INT)));
            return;
        }
        NodeKind::VAR => {
            let ty = node.var().ty();
            node.set_ty(ty);
            return;
        }
        NodeKind::ADDR => {
            let ty = node.lhs().ty();
            if ty.kind() == Some(TypeKind::ARRAY) {
                node.set_ty(TyWrap::point_to(ty.base()));
            } else {
                node.set_ty(TyWrap::point_to(ty));
            }
            return;
        }
        NodeKind::DEREF => {
            if node.lhs().ty().base().ptr.is_none() {
                error_token(node.token(), "invalid pointer dereference")
            }
            node.set_ty(node.lhs().ty().base());
            return;
        }
        _ => {}
    }
    return;
}

use crate::{
    rvcc::{
        get_obj_ty, get_ty_base, get_ty_kind,
        NodeKind, NodeWrap, Ty, TypeKind,
    },
    utils::error_token,
};

#[allow(dead_code)]
pub fn is_int(ty: &Ty) -> bool {
    return ty.kind == Some(TypeKind::INT);
}

#[allow(dead_code)]
pub fn create_ty(kind: TypeKind) -> Option<*mut Ty> {
    Some(Box::leak(Box::new(Ty::new_with_kind(Some(kind)))))
}

#[allow(dead_code)]
pub fn add_ty(node: NodeWrap) {
    if node.ptr.is_none() || !node.ty().is_none() {
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
            let kind = get_ty_kind(node.lhs().ty());
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
            node.set_ty(create_ty(TypeKind::INT));
            return;
        }
        NodeKind::VAR => {
            let ty = get_obj_ty(node.var());
            node.set_ty(ty);
            return;
        }
        NodeKind::ADDR => {
            let ty = node.lhs().ty();
            if get_ty_kind(ty) == Some(TypeKind::ARRAY) {
                node.set_ty(Some(Box::leak(Box::new(Ty::point_to(get_ty_base(ty))))));
            } else {
                node.set_ty(Some(Box::leak(Box::new(Ty::point_to(ty)))));
            }
            return;
        }
        NodeKind::DEREF => {
            if get_ty_base(node.lhs().ty()).is_none() {
                error_token(node.token(), "invalid pointer dereference")
            }
            node.set_ty(get_ty_base(node.lhs().ty()));
            return;
        }
        _ => {}
    }
    return;
}

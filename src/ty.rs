use crate::{
    rvcc::{
        get_node_args, get_node_body, get_node_cond, get_node_els, get_node_inc, get_node_init,
        get_node_kind, get_node_lhs, get_node_next, get_node_rhs, get_node_then, get_node_token,
        get_node_ty, get_node_var, get_obj_ty, get_ty_base, get_ty_kind, set_node_ty, Node,
        NodeKind, Ty, TypeKind,
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
pub fn add_ty(node: Option<*mut Node>) {
    if node.is_none() || !get_node_ty(node).is_none() {
        return;
    }

    add_ty(get_node_lhs(node));
    add_ty(get_node_rhs(node));
    add_ty(get_node_cond(node));
    add_ty(get_node_then(node));
    add_ty(get_node_els(node));
    add_ty(get_node_init(node));
    add_ty(get_node_inc(node));

    let mut next = get_node_body(node);
    while !next.is_none() {
        add_ty(next);
        next = get_node_next(next);
    }

    let mut next = get_node_args(node);
    while !next.is_none() {
        add_ty(next);
        next = get_node_next(next);
    }

    match get_node_kind(node) {
        NodeKind::NEG | NodeKind::Div | NodeKind::Mul | NodeKind::Sub | NodeKind::Add => {
            set_node_ty(
                node,
                get_node_ty(get_node_lhs(node)),
            );
            return;
        }
        NodeKind::ASSIGN => {
            let kind = get_ty_kind(get_node_ty(get_node_lhs(node)));
            if kind == Some(TypeKind::ARRAY) {
                error_token(get_node_token(get_node_lhs(node)).get_ref(), "not an lvalue");
            }
            set_node_ty(
                node,
                get_node_ty(get_node_lhs(node)),
            );
            return;
        }
        NodeKind::FUNCALL
        | NodeKind::EQ
        | NodeKind::NE
        | NodeKind::LT
        | NodeKind::LE
        | NodeKind::Num => {
            set_node_ty(node, create_ty(TypeKind::INT));
            return;
        }
        NodeKind::VAR => {
            let ty = get_obj_ty(get_node_var(node));
            set_node_ty(node, ty);
            return;
        }
        NodeKind::ADDR => {
            let ty = get_node_ty(get_node_lhs(node));
            if get_ty_kind(ty) == Some(TypeKind::ARRAY) {
                set_node_ty(
                    node,
                    Some(Box::leak(Box::new(Ty::point_to(get_ty_base(ty))))),
                );
            } else {
                set_node_ty(node, Some(Box::leak(Box::new(Ty::point_to(ty)))));
            }
            return;
        }
        NodeKind::DEREF => {
            if get_ty_base(get_node_ty(get_node_lhs(node))).is_none() {
                error_token(
                    get_node_token(node).get_ref(),
                    "invalid pointer dereference",
                )
            }
            set_node_ty(
                node,
                get_ty_base(get_node_ty(get_node_lhs(node))),
            );
            return;
        }
        _ => {}
    }
    return;
}

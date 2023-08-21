use crate::{
    rvcc::{
        get_node_body, get_node_cond, get_node_els, get_node_inc, get_node_init, get_node_kind,
        get_node_lhs, get_node_next, get_node_rhs, get_node_then, get_node_token, get_node_ty,
        get_node_var, get_obj_ty, get_ty_base, get_ty_kind, set_node_ty, Node, NodeKind, Ty,
        TypeKind,
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
    if node.is_none() || !get_node_ty(node.unwrap()).is_none() {
        return;
    }

    add_ty(get_node_lhs(node.unwrap()));
    add_ty(get_node_rhs(node.unwrap()));
    add_ty(get_node_cond(node.unwrap()));
    add_ty(get_node_then(node.unwrap()));
    add_ty(get_node_els(node.unwrap()));
    add_ty(get_node_init(node.unwrap()));
    add_ty(get_node_inc(node.unwrap()));

    let mut next = get_node_body(node.unwrap());
    while !next.is_none() {
        add_ty(next);
        next = get_node_next(next.unwrap());
    }

    match get_node_kind(node.unwrap()) {
        NodeKind::ASSIGN
        | NodeKind::NEG
        | NodeKind::Div
        | NodeKind::Mul
        | NodeKind::Sub
        | NodeKind::Add => {
            set_node_ty(
                node.unwrap(),
                get_node_ty(get_node_lhs(node.unwrap()).unwrap()),
            );
            return;
        }
        NodeKind::FUNCALL
        | NodeKind::EQ
        | NodeKind::NE
        | NodeKind::LT
        | NodeKind::LE
        | NodeKind::Num => {
            set_node_ty(node.unwrap(), create_ty(TypeKind::INT));
            return;
        }
        NodeKind::VAR => {
            let ty = get_obj_ty(get_node_var(node.unwrap()));
            set_node_ty(node.unwrap(), ty);
            return;
        }
        NodeKind::ADDR => {
            let ty = Box::leak(Box::new(Ty::point_to(get_node_ty(
                get_node_lhs(node.unwrap()).unwrap(),
            ))));
            set_node_ty(node.unwrap(), Some(ty));
            return;
        }
        NodeKind::DEREF => {
            if get_ty_kind(get_node_ty(get_node_lhs(node.unwrap()).unwrap())) != Some(TypeKind::PTR)
            {
                error_token(
                    get_node_token(node.unwrap()).get_ref(),
                    "invalid pointer dereference",
                )
            }
            set_node_ty(
                node.unwrap(),
                get_ty_base(get_node_ty(get_node_lhs(node.unwrap()).unwrap()).unwrap()),
            );
            return;
        }
        _ => {}
    }
    return;
}

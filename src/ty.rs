use crate::rvcc::{
    get_node_body, get_node_cond, get_node_els, get_node_inc, get_node_init, get_node_kind,
    get_node_lhs, get_node_next, get_node_rhs, get_node_then, get_node_ty, get_ty_base,
    get_ty_kind, set_node_kind, set_node_ty, Node, NodeKind, Ty, TypeKind,
};

#[allow(dead_code)]
pub fn is_int(ty: &Ty) -> bool {
    return ty.kind == Some(TypeKind::INT);
}

#[allow(dead_code)]
pub fn add_ty(node: Option<*mut Node>) {
    if !node.is_none() || !get_node_ty(node.unwrap()).is_none() {
        return;
    }

    add_ty(Some(get_node_lhs(node.unwrap())));
    add_ty(Some(get_node_rhs(node.unwrap())));
    add_ty(get_node_cond(node.unwrap()));
    add_ty(get_node_then(node.unwrap()));
    add_ty(get_node_els(node.unwrap()));
    add_ty(get_node_init(node.unwrap()));
    add_ty(get_node_inc(node.unwrap()));

    let mut next = get_node_body(node.unwrap());
    loop {
        if next.is_none() {
            break;
        }
        add_ty(next);
        next = get_node_next(next.unwrap());
    }

    match get_node_kind(node.unwrap()) {
        NodeKind::Add => {}
        NodeKind::Sub => {}
        NodeKind::Mul => {}
        NodeKind::Div => {}
        NodeKind::NEG => {}
        NodeKind::ASSIGN => {
            set_node_kind(node.unwrap(), get_node_kind(get_node_lhs(node.unwrap())));
            return;
        }
        NodeKind::EQ => {}
        NodeKind::NE => {}
        NodeKind::LT => {}
        NodeKind::LE => {}
        NodeKind::VAR => {}
        NodeKind::Num => {
            set_node_ty(
                node.unwrap(),
                Some(Box::leak(Box::new(Ty::new_with_kind(TypeKind::INT)))),
            );
            return;
        }
        NodeKind::ADDR => {
            set_node_ty(
                node.unwrap(),
                Some(Box::leak(Box::new(Ty::point_to(
                    get_node_ty(get_node_lhs(node.unwrap())).unwrap(),
                )))),
            );
            return;
        }
        NodeKind::DEREF => {
            if get_ty_kind(get_node_ty(get_node_lhs(node.unwrap())).unwrap()) == Some(TypeKind::PTR)
            {
                set_node_ty(
                    node.unwrap(),
                    get_ty_base(get_node_ty(get_node_lhs(node.unwrap())).unwrap()),
                )
            } else {
                set_node_ty(
                    node.unwrap(),
                    Some(Box::leak(Box::new(Ty::new_with_kind(TypeKind::INT)))),
                );
            }
            return;
        }
        _ => {}
    }
}

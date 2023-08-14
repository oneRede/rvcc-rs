use crate::{
    rvcc::{
        get_node_next, get_obj_name, get_obj_next, set_node_body, set_node_cond, set_node_els,
        set_node_next, set_node_then, Function, Node, NodeKind, Obj, TokenKind, TokenWrap, set_node_init, set_node_inc,
    },
    tokenize::{equal, error_token, skip, str_to_chars},
};

pub static mut LOCALS: Option<*mut Obj> = None;

#[allow(dead_code)]
pub fn expr(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    return assign(token);
}

#[allow(dead_code)]
pub fn create_binary_node(kind: NodeKind, lhs: *mut Node, rhs: *mut Node) -> *mut Node {
    Box::leak(Box::new(Node::new_binary(kind, lhs, rhs)))
}

#[allow(dead_code)]
pub fn create_unary_node(kind: NodeKind, expr: *mut Node) -> *mut Node {
    Box::leak(Box::new(Node::new_unary(kind, expr)))
}

#[allow(dead_code)]
pub fn create_node(kind: NodeKind) -> *mut Node {
    Box::leak(Box::new(Node::new(kind)))
}

#[allow(dead_code)]
pub fn assign(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = equality(token);
    if equal(token.get_ref(), &['=']) {
        let (n, t) = assign(token.set(token.get_next()));
        node = Some(create_binary_node(
            NodeKind::ASSIGN,
            node.unwrap(),
            n.unwrap(),
        ));
        token = t;
    }

    return (node, token);
}

#[allow(dead_code)]
fn equality(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = relational(token);

    loop {
        if equal(token.get_ref(), &['=', '=']) {
            let (n, t) = relational(token.set(token.get_next()));
            node = Some(create_binary_node(NodeKind::EQ, node.unwrap(), n.unwrap()));
            token = t;
            continue;
        }
        if equal(token.get_ref(), &['!', '=']) {
            let (n, t) = relational(token.set(token.get_next()));
            node = Some(create_binary_node(NodeKind::NE, node.unwrap(), n.unwrap()));
            token = t;
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
fn relational(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = add(token);

    loop {
        if equal(token.get_ref(), &['<']) {
            let (n, t) = add(token.set(token.get_next()));
            node = Some(create_binary_node(NodeKind::LT, node.unwrap(), n.unwrap()));
            token = t;
            continue;
        }

        if equal(token.get_ref(), &['<', '=']) {
            let (n, t) = add(token.set(token.get_next()));
            node = Some(create_binary_node(NodeKind::LE, node.unwrap(), n.unwrap()));
            token = t;
            continue;
        }

        if equal(token.get_ref(), &['>']) {
            let (n, t) = add(token.set(token.get_next()));
            node = Some(create_binary_node(NodeKind::LT, n.unwrap(), node.unwrap()));
            token = t;
            continue;
        }

        if equal(token.get_ref(), &['>', '=']) {
            let (n, t) = add(token.set(token.get_next()));
            node = Some(create_binary_node(NodeKind::LE, n.unwrap(), node.unwrap()));
            token = t;
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
fn add(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = mul(token);

    loop {
        if equal(token.get_ref(), &['+']) {
            let (n, t) = mul(token.set(token.get_next()));
            node = Some(create_binary_node(NodeKind::Add, node.unwrap(), n.unwrap()));
            token = t;
            continue;
        }
        if equal(token.get_ref(), &['-']) {
            let (n, t) = mul(token.set(token.get_next()));
            node = Some(create_binary_node(NodeKind::Sub, node.unwrap(), n.unwrap()));
            token = t;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn mul(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = unary(token);

    loop {
        if equal(token.get_ref(), &['*']) {
            let (n, t) = unary(token.set(token.get_next()));
            node = Some(create_binary_node(NodeKind::Mul, node.unwrap(), n.unwrap()));
            token = t;
            continue;
        }
        if equal(token.get_ref(), &['/']) {
            let (n, t) = unary(token.set(token.get_next()));
            node = Some(create_binary_node(NodeKind::Div, node.unwrap(), n.unwrap()));
            token = t;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn unary(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    if equal(token.get_ref(), &['+']) {
        return unary(token.set(token.get_next()));
    }
    if equal(token.get_ref(), &['-']) {
        let (n, t) = unary(token.set(token.get_next()));
        return (Some(create_unary_node(NodeKind::NEG, n.unwrap())), t);
    }

    primary(token)
}

#[allow(dead_code)]
fn primary(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    if equal(token.get_ref(), &['(']) {
        let (n, t) = expr(token.set(token.get_next()));
        token = t;
        return (n, token.set(token.get_next()));
    }

    if token.get_kind() == TokenKind::IDENT {
        let mut var = find_var(token);
        if var.is_none() {
            let len = token.get_len();
            let name: String = token.get_loc().unwrap()[..len].iter().collect();
            var = Some(Obj::new(Box::leak(Box::new(name))));
        }
        let node = Box::leak(Box::new(Node::new_var_node(var)));
        return (Some(node), token.set(token.get_next()));
    }

    if token.get_kind() == TokenKind::Num {
        let node = Box::leak(Box::new(Node::new_num(token.get_val() as i64)));
        return (Some(node), token.set(token.get_next()));
    }

    error_token(token.get_ref(), "expected an expression");
    (None, token)
}

#[allow(dead_code)]
pub fn compound_stmt(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let head: *mut Node = create_node(NodeKind::Num);
    let mut cur = head;

    loop {
        if equal(token.get_ref(), &['}']) {
            break;
        }
        let (n, t) = stmt(token);
        token.set(t.ptr.unwrap());
        set_node_next(cur, n);
        cur = get_node_next(cur).unwrap();
    }

    let node: *mut Node = create_node(NodeKind::BLOCK);
    set_node_body(node, get_node_next(head));
    return (Some(node), token.set(token.get_next()));
}

#[allow(dead_code)]
fn stmt(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    if equal(token.get_ref(), str_to_chars("return")) {
        let (n, t) = expr(token.set(token.get_next()));

        let node = create_unary_node(NodeKind::RETURN, n.unwrap());
        token.set(skip(t.get_ref(), &[';']).unwrap());
        return (Some(node), token);
    }

    if equal(token.get_ref(), str_to_chars("if")) {
        let node: *mut Node = create_node(NodeKind::IF);

        token.set(token.get_next());
        token.set(skip(token.get_ref(), &['(']).unwrap());

        let (n, t) = expr(token);
        set_node_cond(node, n);

        token.set(skip(t.get_ref(), &[')']).unwrap());
        let (n, t) = stmt(token);
        token = t;
        set_node_then(node, n);

        if equal(token.get_ref(), str_to_chars("else")) {
            token.set(token.get_next());
            let (n, t) = stmt(token);
            set_node_els(node, n);
            token = t;
        }
        return (Some(node), token);
    }

    if equal(token.get_ref(), str_to_chars("for")){
        let node: *mut Node = create_node(NodeKind::FOR);

        token.set(token.get_next());
        token.set(skip(token.get_ref(), &['(']).unwrap());

        let (n, mut token) = expr_stmt(token);
        set_node_init(node, n);

        if !equal(token.get_ref(), &[';']) {
            let (n, t) = expr(token);
            set_node_cond(node, n);
            token = t;
        }
        token.set(skip(token.get_ref(), &[';']).unwrap());

        if !equal(token.get_ref(), &[')']) {
            let (n, t) = expr(token);
            set_node_inc(node, n);
            token = t;
        }
        token.set(skip(token.get_ref(), &[')']).unwrap());

        let (n, token) = stmt(token);
        set_node_then(node, n);

        return (Some(node), token)

    }

    if equal(token.get_ref(), str_to_chars("while")){
        let node: *mut Node = create_node(NodeKind::FOR);

        token.set(token.get_next());
        token.set(skip(token.get_ref(), &['(']).unwrap());

        let (n, mut token) = expr(token);
        set_node_cond(node, n);
        token.set(skip(token.get_ref(), &[')']).unwrap());

        let (n, token) = stmt(token);
        set_node_then(node, n);

        return (Some(node), token)
    }

    if equal(token.get_ref(), &['{']) {
        return compound_stmt(token.set(token.get_next()));
    }
    expr_stmt(token)
}

#[allow(dead_code)]
fn expr_stmt(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    if equal(token.get_ref(), &[';']) {
        token.set(token.get_next());
        return (Some(create_node(NodeKind::BLOCK)), token);
    }

    let (n, t) = expr(token);
    let node = create_unary_node(NodeKind::ExprStmt, n.unwrap());
    token.set(skip(t.get_ref(), &[';']).unwrap());
    return (Some(node), token);
}

#[allow(dead_code)]
pub fn parse(mut token: TokenWrap) -> *mut Function {
    token.set(skip(token.get_ref(), &['{']).unwrap());

    let mut prog = Function::empty();
    let (n, _t) = compound_stmt(token);
    prog.body = n;
    prog.locals = unsafe { LOCALS };
    return Box::leak(Box::new(prog));
}

#[allow(dead_code)]
pub fn find_var(token: TokenWrap) -> Option<*mut Obj> {
    if unsafe { LOCALS.is_none() } {
        return None;
    }
    let mut var = unsafe { LOCALS.unwrap() };
    loop {
        let name: Vec<char> = get_obj_name(var).chars().into_iter().collect();
        if get_obj_name(var).len() == token.get_len() && equal(token.get_ref(), &name) {
            return Some(var);
        }
        if get_obj_next(var).is_none() {
            break;
        }
        var = get_obj_next(var).unwrap();
    }
    None
}

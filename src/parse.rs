use crate::{
    rvcc::{
        get_function_next, get_node_next, get_node_ty, get_obj_name, get_obj_next, get_ty_base,
        get_ty_ref, get_ty_token, set_function_next, set_node_args, set_node_body, set_node_cond,
        set_node_els, set_node_func_name, set_node_inc, set_node_init, set_node_next,
        set_node_then, set_node_ty, set_ty_token, Function, Node, NodeKind, Obj, TokenKind,
        TokenWrap, Ty, TypeKind,
    },
    tokenize::{consume, equal, skip, str_to_chars},
    ty::{add_ty, create_ty, is_int},
    utils::error_token,
};

pub static mut LOCALS: Option<*mut Obj> = None;

#[allow(dead_code)]
pub fn create_binary_node(kind: NodeKind, lhs: *mut Node, rhs: *mut Node) -> *mut Node {
    Box::leak(Box::new(Node::new_binary(kind, lhs, rhs)))
}

#[allow(dead_code)]
pub fn create_binary_node_v2(
    kind: NodeKind,
    lhs: *mut Node,
    rhs: *mut Node,
    token: TokenWrap,
) -> *mut Node {
    Box::leak(Box::new(Node::new_binary_v2(kind, lhs, rhs, token)))
}

#[allow(dead_code)]
pub fn create_unary_node(kind: NodeKind, expr: *mut Node) -> *mut Node {
    Box::leak(Box::new(Node::new_unary(kind, expr)))
}

#[allow(dead_code)]
pub fn create_unary_node_v2(
    kind: NodeKind,
    expr: Option<*mut Node>,
    token: TokenWrap,
) -> *mut Node {
    Box::leak(Box::new(Node::new_unary_v2(kind, expr, token)))
}

#[allow(dead_code)]
pub fn create_node(kind: NodeKind) -> *mut Node {
    Box::leak(Box::new(Node::new(kind)))
}

#[allow(dead_code)]
pub fn create_num_node_v2(val: i64, token: TokenWrap) -> *mut Node {
    Box::leak(Box::new(Node::new_num_v2(val, token)))
}

#[allow(dead_code)]
pub fn create_var_node_v2(obj: Option<*mut Obj>, token: TokenWrap) -> *mut Node {
    Box::leak(Box::new(Node::new_var_node_v2(obj, token)))
}

#[allow(dead_code)]
pub fn create_node_v2(kind: NodeKind, token: TokenWrap) -> *mut Node {
    Box::leak(Box::new(Node::new_v2(kind, token)))
}

#[allow(dead_code)]
pub fn expr(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    return assign(token);
}

#[allow(dead_code)]
pub fn assign(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = equality(token);
    if equal(token.get_ref(), &['=']) {
        let (n, t) = assign(token.set(token.get_next()));
        node = Some(create_binary_node_v2(
            NodeKind::ASSIGN,
            node.unwrap(),
            n.unwrap(),
            token,
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
            node = Some(create_binary_node_v2(
                NodeKind::EQ,
                node.unwrap(),
                n.unwrap(),
                t,
            ));
            token = t;
            continue;
        }
        if equal(token.get_ref(), &['!', '=']) {
            let (n, t) = relational(token.set(token.get_next()));
            node = Some(create_binary_node_v2(
                NodeKind::NE,
                node.unwrap(),
                n.unwrap(),
                t,
            ));
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
            node = Some(create_binary_node_v2(
                NodeKind::LT,
                node.unwrap(),
                n.unwrap(),
                t,
            ));
            token = t;
            continue;
        }

        if equal(token.get_ref(), &['<', '=']) {
            let (n, t) = add(token.set(token.get_next()));
            node = Some(create_binary_node_v2(
                NodeKind::LE,
                node.unwrap(),
                n.unwrap(),
                t,
            ));
            token = t;
            continue;
        }

        if equal(token.get_ref(), &['>']) {
            let (n, t) = add(token.set(token.get_next()));
            node = Some(create_binary_node_v2(
                NodeKind::LT,
                n.unwrap(),
                node.unwrap(),
                t,
            ));
            token = t;
            continue;
        }

        if equal(token.get_ref(), &['>', '=']) {
            let (n, t) = add(token.set(token.get_next()));
            node = Some(create_binary_node_v2(
                NodeKind::LE,
                n.unwrap(),
                node.unwrap(),
                t,
            ));
            token = t;
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
pub fn new_add(
    mut lhs: Option<*mut Node>,
    mut rhs: Option<*mut Node>,
    token: TokenWrap,
) -> (Option<*mut Node>, TokenWrap) {
    add_ty(lhs);
    add_ty(rhs);

    if is_int(get_ty_ref(get_node_ty(lhs.unwrap())))
        && is_int(get_ty_ref(get_node_ty(rhs.unwrap())))
    {
        let node = create_binary_node_v2(NodeKind::Add, lhs.unwrap(), rhs.unwrap(), token);
        return (Some(node), token);
    }
    if !get_ty_base(get_node_ty(lhs.unwrap()).unwrap()).is_none()
        && !get_ty_base(get_node_ty(rhs.unwrap()).unwrap()).is_none()
    {
        error_token(token.get_ref(), "invalid operands")
    }
    if get_ty_base(get_node_ty(lhs.unwrap()).unwrap()).is_none()
        && !get_ty_base(get_node_ty(rhs.unwrap()).unwrap()).is_none()
    {
        let tmp = lhs;
        lhs = rhs;
        rhs = tmp;
    }
    let num_node = create_num_node_v2(8, token);
    let rhs = create_binary_node_v2(NodeKind::Mul, rhs.unwrap(), num_node, token);
    let node = create_binary_node_v2(NodeKind::Add, lhs.unwrap(), rhs, token);
    return (Some(node), token);
}

#[allow(dead_code)]
pub fn new_sub(
    lhs: Option<*mut Node>,
    rhs: Option<*mut Node>,
    token: TokenWrap,
) -> (Option<*mut Node>, TokenWrap) {
    add_ty(lhs);
    add_ty(rhs);

    if is_int(get_ty_ref(get_node_ty(lhs.unwrap())))
        && is_int(get_ty_ref(get_node_ty(rhs.unwrap())))
    {
        let node = create_binary_node_v2(NodeKind::Sub, lhs.unwrap(), rhs.unwrap(), token);
        return (Some(node), token);
    }

    if !(get_ty_base(get_node_ty(lhs.unwrap()).unwrap()).is_none())
        && is_int(get_ty_ref(get_node_ty(rhs.unwrap())))
    {
        let num_node = create_num_node_v2(8, token);
        let rhs_node = create_binary_node_v2(NodeKind::Mul, rhs.unwrap(), num_node, token);
        add_ty(Some(rhs_node));
        let node = create_binary_node_v2(NodeKind::Sub, lhs.unwrap(), rhs_node, token);
        set_node_ty(node, get_node_ty(lhs.unwrap()));
        return (Some(node), token);
    }
    if !get_ty_base(get_node_ty(lhs.unwrap()).unwrap()).is_none()
        && !get_ty_base(get_node_ty(rhs.unwrap()).unwrap()).is_none()
    {
        let node = create_binary_node_v2(NodeKind::Sub, lhs.unwrap(), rhs.unwrap(), token);
        let ty = create_ty(TypeKind::INT);
        set_node_ty(node, ty);
        let num_node = create_num_node_v2(8, token);
        let node = create_binary_node_v2(NodeKind::Div, node, num_node, token);
        return (Some(node), token);
    }
    error_token(token.get_ref(), "invalid operands");
    return (None, token);
}

#[allow(dead_code)]
fn add(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = mul(token);

    loop {
        if equal(token.get_ref(), &['+']) {
            let (n, t) = mul(token.set(token.get_next()));
            let (n, t) = new_add(node, n, t);
            node = n;
            token = t;
            continue;
        }
        if equal(token.get_ref(), &['-']) {
            let (n, t) = mul(token.set(token.get_next()));
            let (n, t) = new_sub(node, n, t);
            node = n;
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
        let start = token;
        if equal(token.get_ref(), &['*']) {
            let (n, t) = unary(token.set(token.get_next()));
            node = Some(create_binary_node_v2(
                NodeKind::Mul,
                node.unwrap(),
                n.unwrap(),
                start,
            ));
            token = t;
            continue;
        }
        if equal(token.get_ref(), &['/']) {
            let (n, t) = unary(token.set(token.get_next()));
            node = Some(create_binary_node_v2(
                NodeKind::Div,
                node.unwrap(),
                n.unwrap(),
                start,
            ));
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
        return (Some(create_unary_node_v2(NodeKind::NEG, n, t)), t);
    }
    if equal(token.get_ref(), &['&']) {
        let (n, t) = unary(token.set(token.get_next()));
        return (Some(create_unary_node_v2(NodeKind::ADDR, n, t)), t);
    }
    if equal(token.get_ref(), &['*']) {
        let (n, t) = unary(token.set(token.get_next()));
        return (Some(create_unary_node_v2(NodeKind::DEREF, n, t)), t);
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
        if equal(unsafe { token.get_next().as_ref().unwrap() }, &['(']) {
            return func_call(token);
        }

        let var = find_var(token);
        if var.is_none() {
            error_token(token.get_ref(), "undefined variable");
        }
        let node = create_var_node_v2(var, token);
        return (Some(node), token.set(token.get_next()));
    }

    if token.get_kind() == TokenKind::Num {
        let node = create_num_node_v2(token.get_val() as i64, token);
        return (Some(node), token.set(token.get_next()));
    }

    error_token(token.get_ref(), "expected an expression");
    (None, token)
}

#[allow(dead_code)]
pub fn compound_stmt(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let head: *mut Node = create_node_v2(NodeKind::Num, token);
    let mut cur = head;

    while !equal(token.get_ref(), &['}']) {
        if equal(token.get_ref(), str_to_chars("int")) {
            let dec = declaration(token);
            token = dec.1;
            set_node_next(cur, dec.0)
        } else {
            let (n, t) = stmt(token);
            token = t;
            set_node_next(cur, n);
        }

        cur = get_node_next(cur).unwrap();
        add_ty(Some(cur));
    }

    let node: *mut Node = create_node(NodeKind::BLOCK);
    set_node_body(node, get_node_next(head));
    return (Some(node), token.set(token.get_next()));
}

#[allow(dead_code)]
fn stmt(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    if equal(token.get_ref(), str_to_chars("return")) {
        let (n, t) = expr(token.set(token.get_next()));

        let node = create_unary_node_v2(NodeKind::RETURN, n, t);
        token.set(skip(t.get_ref(), &[';']).unwrap());
        return (Some(node), token);
    }

    if equal(token.get_ref(), str_to_chars("if")) {
        let node: *mut Node = create_node_v2(NodeKind::IF, token);

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

    if equal(token.get_ref(), str_to_chars("for")) {
        let node: *mut Node = create_node_v2(NodeKind::FOR, token);

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

        return (Some(node), token);
    }

    if equal(token.get_ref(), str_to_chars("while")) {
        let node: *mut Node = create_node_v2(NodeKind::FOR, token);

        token.set(token.get_next());
        token.set(skip(token.get_ref(), &['(']).unwrap());

        let (n, mut token) = expr(token);
        set_node_cond(node, n);
        token.set(skip(token.get_ref(), &[')']).unwrap());

        let (n, token) = stmt(token);
        set_node_then(node, n);

        return (Some(node), token);
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
        return (Some(create_node_v2(NodeKind::BLOCK, token)), token);
    }

    let (n, t) = expr(token);
    let node = create_unary_node_v2(NodeKind::ExprStmt, n, token);
    token.set(skip(t.get_ref(), &[';']).unwrap());
    return (Some(node), token);
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

#[allow(dead_code)]
pub fn get_ident(token: TokenWrap) -> &'static str {
    if token.get_kind() != TokenKind::IDENT {
        error_token(token.get_ref(), "expected an identifier");
    }

    let len = token.get_len();
    let name: String = token.get_loc().unwrap()[..len].iter().collect();
    Box::leak(Box::new(name))
}

#[allow(dead_code)]
pub fn declspec(mut token: TokenWrap) -> (TokenWrap, Option<*mut Ty>) {
    token.set(skip(token.get_ref(), str_to_chars("int")).unwrap());
    return (token, create_ty(TypeKind::INT));
}

#[allow(dead_code)]
pub fn declarator(mut token: TokenWrap, mut ty: Option<*mut Ty>) -> (Option<*mut Ty>, TokenWrap) {
    while consume(token, "*").0 {
        token = consume(token, "*").1;
        ty = Some(Box::leak(Box::new(Ty::point_to(ty))));
    }

    if token.get_kind() != TokenKind::IDENT {
        error_token(token.get_ref(), "expected a variable name");
    }

    let (typ, tk) = ty_suffix(token, ty);
    ty = typ;
    token = tk;
    set_ty_token(ty.unwrap(), token);

    return (ty, token);
}

#[allow(dead_code)]
pub fn declaration(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let base_ty = declspec(token).1;
    token = declspec(token).0;

    let head: *mut Node = create_node_v2(NodeKind::Num, token);
    let mut cur = head;

    let mut i = 0;
    while !equal(token.get_ref(), &[';']) {
        if i > 0 {
            token.set(skip(token.get_ref(), &[',']).unwrap());
        }
        i += 1;

        let ty = declarator(token, base_ty).0;
        token = declarator(token, base_ty).1;
        let var = Obj::new(get_ident(get_ty_token(ty.unwrap())), ty);

        if !equal(token.get_ref(), &['=']) {
            continue;
        }

        let lhs = create_var_node_v2(Some(var), get_ty_token(ty.unwrap()));
        let rhs = assign(token.set(token.get_next()));
        token = rhs.1;
        let node = create_binary_node_v2(NodeKind::ASSIGN, lhs, rhs.0.unwrap(), token);

        set_node_next(
            cur,
            Some(create_unary_node_v2(NodeKind::ExprStmt, Some(node), token)),
        );
        cur = get_node_next(cur).unwrap();
    }

    let node = create_node_v2(NodeKind::BLOCK, token);
    set_node_body(node, get_node_next(head));
    token.set(token.get_next());

    return (Some(node), token);
}

#[allow(dead_code)]
pub fn func_call(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let start = token;
    token.set(token.get_next());
    token.set(token.get_next());

    let head: *mut Node = create_node_v2(NodeKind::Num, token);
    let mut cur = head;

    while !equal(token.get_ref(), &[')']) {
        if cur != head {
            token.set(skip(token.get_ref(), &[',']).unwrap());
        }
        let (n, t) = assign(token);
        set_node_next(cur, n);
        cur = get_node_next(cur).unwrap();
        token = t;
    }
    token.set(skip(token.get_ref(), &[')']).unwrap());

    let node = create_node_v2(NodeKind::FUNCALL, start);
    let len = start.get_len();
    let func_name: String = start.get_loc().unwrap()[..len].iter().collect();
    set_node_func_name(node, Box::leak(Box::new(func_name)));

    set_node_args(node, get_node_next(head));

    return (Some(node), token);
}

#[allow(dead_code)]
pub fn ty_suffix(mut token: TokenWrap, ty: Option<*mut Ty>) -> (Option<*mut Ty>, TokenWrap) {
    if equal(token.get_ref(), &['(']) {
        token.set(skip(token.get_ref(), &[')']).unwrap());
        return (Some(Box::leak(Box::new(Ty::new_func_ty(ty)))), token);
    }

    return (ty, token);
}

#[allow(dead_code)]
pub fn function(token: TokenWrap) -> (Option<*mut Function>, TokenWrap) {
    let (typ, tk) = declspec(token);
    let (_typ, tk) = declarator(typ, tk);

    unsafe { LOCALS = None };

    let mut func = Function::empty();
    func.name = get_ident(tk);
    func.locals = unsafe { LOCALS };

    return (Some(Box::leak(Box::new(func))), tk);
}

#[allow(dead_code)]
pub fn parse(token: TokenWrap) -> Option<*mut Function> {
    let head: Option<*mut Function> = Some(Box::leak(Box::new(Function::empty())));
    let mut cur = head;

    while token.get_kind() != TokenKind::EOF {
        let (f, _tk) = function(token);
        set_function_next(cur.unwrap(), f);
        cur = get_function_next(cur.unwrap());
    }

    return get_function_next(head.unwrap());
}

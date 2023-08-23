use crate::{
    rvcc::{
        get_function_next, get_node_next, get_node_ty, get_obj_name, get_obj_next, get_ty_base,
        get_ty_next, get_ty_params, get_ty_ref, get_ty_size, get_ty_token, set_function_next,
        set_node_args, set_node_body, set_node_cond, set_node_els, set_node_func_name,
        set_node_inc, set_node_init, set_node_lhs, set_node_next, set_node_then, set_node_ty,
        set_ty_next, set_ty_params, set_ty_token, Function, Node, NodeKind, Obj, TokenKind,
        TokenWrap, Ty, TypeKind,
    },
    tokenize::{consume, equal, skip},
    ty::{add_ty, create_ty, is_int},
    utils::error_token,
};

pub static mut LOCALS: Option<*mut Obj> = None;

#[allow(dead_code)]
pub fn create_binary_node(
    kind: NodeKind,
    lhs: Option<*mut Node>,
    rhs: Option<*mut Node>,
    token: TokenWrap,
) -> Option<*mut Node> {
    Some(Box::leak(Box::new(Node::new_binary(
        kind, lhs, rhs, token,
    ))))
}

#[allow(dead_code)]
pub fn create_unary_node(
    kind: NodeKind,
    expr: Option<*mut Node>,
    token: TokenWrap,
) -> Option<*mut Node> {
    Some(Box::leak(Box::new(Node::new_unary(kind, expr, token))))
}

#[allow(dead_code)]
pub fn create_num_node(val: i64, token: TokenWrap) -> Option<*mut Node> {
    Some(Box::leak(Box::new(Node::new_num(val, token))))
}

#[allow(dead_code)]
pub fn create_var_node(obj: Option<*mut Obj>, token: TokenWrap) -> Option<*mut Node> {
    Some(Box::leak(Box::new(Node::new_var_node(obj, token))))
}

#[allow(dead_code)]
pub fn create_node(kind: NodeKind, token: TokenWrap) -> Option<*mut Node> {
    Some(Box::leak(Box::new(Node::new(kind, token))))
}

#[allow(dead_code)]
pub fn expr(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    return assign(token);
}

#[allow(dead_code)]
pub fn assign(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = equality(token);
    if equal(token, "=") {
        let (n, t) = assign(token.reset_by_next());
        node = create_binary_node(NodeKind::ASSIGN, node, n, token);
        token = t;
    }

    return (node, token);
}

#[allow(dead_code)]
fn equality(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = relational(token);

    loop {
        if equal(token, "==") {
            let (n, t) = relational(token.reset_by_next());
            node = create_binary_node(NodeKind::EQ, node, n, t);
            token = t;
            continue;
        }
        if equal(token, "!=") {
            let (n, t) = relational(token.reset_by_next());
            node = create_binary_node(NodeKind::NE, node, n, t);
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
        if equal(token, "<") {
            let (n, t) = add(token.reset_by_next());
            node = create_binary_node(NodeKind::LT, node, n, t);
            token = t;
            continue;
        }

        if equal(token, "<=") {
            let (n, t) = add(token.reset_by_next());
            node = create_binary_node(NodeKind::LE, node, n, t);
            token = t;
            continue;
        }

        if equal(token, ">") {
            let (n, t) = add(token.reset_by_next());
            node = create_binary_node(NodeKind::LT, n, node, t);
            token = t;
            continue;
        }

        if equal(token, ">=") {
            let (n, t) = add(token.reset_by_next());
            node = create_binary_node(NodeKind::LE, n, node, t);
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

    if is_int(get_ty_ref(get_node_ty(lhs))) && is_int(get_ty_ref(get_node_ty(rhs))) {
        let node = create_binary_node(NodeKind::Add, lhs, rhs, token);
        return (node, token);
    }
    if !get_ty_base(get_node_ty(lhs)).is_none() && !get_ty_base(get_node_ty(rhs)).is_none() {
        error_token(token, "invalid operands")
    }
    if get_ty_base(get_node_ty(lhs)).is_none() && !get_ty_base(get_node_ty(rhs)).is_none() {
        let tmp = lhs;
        lhs = rhs;
        rhs = tmp;
    }
    let val = get_ty_size(get_ty_base(get_node_ty(lhs)));
    let num_node = create_num_node(val as i64, token);
    let rhs = create_binary_node(NodeKind::Mul, rhs, num_node, token);
    let node = create_binary_node(NodeKind::Add, lhs, rhs, token);
    return (node, token);
}

#[allow(dead_code)]
pub fn new_sub(
    lhs: Option<*mut Node>,
    rhs: Option<*mut Node>,
    token: TokenWrap,
) -> (Option<*mut Node>, TokenWrap) {
    add_ty(lhs);
    add_ty(rhs);

    if is_int(get_ty_ref(get_node_ty(lhs))) && is_int(get_ty_ref(get_node_ty(rhs))) {
        let node = create_binary_node(NodeKind::Sub, lhs, rhs, token);
        return (node, token);
    }

    if !(get_ty_base(get_node_ty(lhs)).is_none()) && is_int(get_ty_ref(get_node_ty(rhs))) {
        let val = get_ty_size(get_ty_base(get_node_ty(lhs)));
        let num_node = create_num_node(val as i64, token);
        let rhs_node = create_binary_node(NodeKind::Mul, rhs, num_node, token);
        add_ty(rhs_node);
        let node = create_binary_node(NodeKind::Sub, lhs, rhs_node, token);
        set_node_ty(node, get_node_ty(lhs));
        return (node, token);
    }
    if !get_ty_base(get_node_ty(lhs)).is_none() && !get_ty_base(get_node_ty(rhs)).is_none() {
        let node = create_binary_node(NodeKind::Sub, lhs, rhs, token);
        let ty = create_ty(TypeKind::INT);
        set_node_ty(node, ty);
        let val = get_ty_size(get_ty_base(get_node_ty(lhs)));
        let num_node = create_num_node(val as i64, token);
        let node = create_binary_node(NodeKind::Div, node, num_node, token);
        return (node, token);
    }
    error_token(token, "invalid operands");
    return (None, token);
}

#[allow(dead_code)]
fn add(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = mul(token);

    loop {
        if equal(token, "+") {
            let (n, t) = mul(token.reset_by_next());
            let (n, t) = new_add(node, n, t);
            node = n;
            token = t;
            continue;
        }
        if equal(token, "-") {
            let (n, t) = mul(token.reset_by_next());
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
        if equal(token, "*") {
            let (n, t) = unary(token.reset_by_next());
            node = create_binary_node(NodeKind::Mul, node, n, start);
            token = t;
            continue;
        }
        if equal(token, "/") {
            let (n, t) = unary(token.reset_by_next());
            node = create_binary_node(NodeKind::Div, node, n, start);
            token = t;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn unary(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    if equal(token, "+") {
        return unary(token.reset_by_next());
    }
    if equal(token, "-") {
        let (n, t) = unary(token.reset_by_next());
        return (create_unary_node(NodeKind::NEG, n, t), t);
    }
    if equal(token, "&") {
        let (n, t) = unary(token.reset_by_next());
        return (create_unary_node(NodeKind::ADDR, n, t), t);
    }
    if equal(token, "*") {
        let (n, t) = unary(token.reset_by_next());
        return (create_unary_node(NodeKind::DEREF, n, t), t);
    }

    postfix(token)
}

#[allow(dead_code)]
fn primary(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    if equal(token, "(") {
        let (n, t) = expr(token.reset_by_next());
        token = t;
        return (n, token.set(skip(token, ")")));
    }

    if token.kind() == TokenKind::IDENT {
        let mut start = token;
        if equal(start.reset_by_next(), "(") {
            return func_call(token);
        }

        let var = find_var(token);
        if var.is_none() {
            error_token(token, "undefined variable");
        }
        let node = create_var_node(var, token);
        return (node, token.reset_by_next());
    }

    if token.kind() == TokenKind::Num {
        let node = create_num_node(token.val() as i64, token);
        token.reset_by_next();
        return (node, token);
    }

    error_token(token, "expected an expression");
    (None, token)
}

#[allow(dead_code)]
pub fn compound_stmt(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let head: Option<*mut Node> = create_node(NodeKind::Num, token);
    let mut cur = head;

    while !equal(token, "}") {
        if equal(token, "int") {
            let dec = declaration(token);
            token = dec.1;
            set_node_next(cur, dec.0)
        } else {
            let (n, t) = stmt(token);
            token = t;
            set_node_next(cur, n);
        }

        cur = get_node_next(cur);
        add_ty(cur);
    }

    let node: Option<*mut Node> = create_node(NodeKind::BLOCK, token);
    set_node_body(node, get_node_next(head));
    return (node, token.reset_by_next());
}

#[allow(dead_code)]
fn stmt(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    if equal(token, "return") {
        let node = create_node(NodeKind::RETURN, token);
        let (n, t) = expr(token.reset_by_next());
        set_node_lhs(node, n);

        token.set(skip(t, ";"));
        return (node, token);
    }

    if equal(token, "if") {
        let node: Option<*mut Node> = create_node(NodeKind::IF, token);

        token.reset_by_next();
        token.set(skip(token, "("));

        let (n, t) = expr(token);
        set_node_cond(node, n);

        token.set(skip(t, ")"));
        let (n, t) = stmt(token);
        token = t;
        set_node_then(node, n);

        if equal(token, "else") {
            token.reset_by_next();
            let (n, t) = stmt(token);
            set_node_els(node, n);
            token = t;
        }
        return (node, token);
    }

    if equal(token, "for") {
        let node: Option<*mut Node> = create_node(NodeKind::FOR, token);

        token.reset_by_next();
        token.set(skip(token, "("));

        let (n, mut token) = expr_stmt(token);
        set_node_init(node, n);

        if !equal(token, ";") {
            let (n, t) = expr(token);
            set_node_cond(node, n);
            token = t;
        }
        token.set(skip(token, ";"));

        if !equal(token, ")") {
            let (n, t) = expr(token);
            set_node_inc(node, n);
            token = t;
        }
        token.set(skip(token, ")"));

        let (n, token) = stmt(token);
        set_node_then(node, n);

        return (node, token);
    }

    if equal(token, "while") {
        let node: Option<*mut Node> = create_node(NodeKind::FOR, token);

        token.reset_by_next();
        token.set(skip(token, "("));

        let (n, mut token) = expr(token);
        set_node_cond(node, n);
        token.set(skip(token, ")"));

        let (n, token) = stmt(token);
        set_node_then(node, n);

        return (node, token);
    }

    if equal(token, "{") {
        return compound_stmt(token.reset_by_next());
    }
    expr_stmt(token)
}

#[allow(dead_code)]
fn expr_stmt(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    if equal(token, ";") {
        token.reset_by_next();
        return (create_node(NodeKind::BLOCK, token), token);
    }

    let (n, t) = expr(token);
    let node = create_unary_node(NodeKind::ExprStmt, n, token);
    token.set(skip(t, ";"));
    return (node, token);
}

#[allow(dead_code)]
pub fn find_var(token: TokenWrap) -> Option<*mut Obj> {
    if unsafe { LOCALS.is_none() } {
        return None;
    }
    let mut var = unsafe { LOCALS };
    loop {
        let name = get_obj_name(var);
        if get_obj_name(var).len() == token.get_len() && equal(token, name) {
            return var;
        }
        if get_obj_next(var).is_none() {
            break;
        }
        var = get_obj_next(var);
    }
    None
}

#[allow(dead_code)]
pub fn get_ident(token: TokenWrap) -> &'static str {
    if token.kind() != TokenKind::IDENT {
        error_token(token, "expected an identifier");
    }

    let len = token.get_len();
    let name: String = token.get_loc().unwrap()[..len].iter().collect();
    Box::leak(Box::new(name))
}

#[allow(dead_code)]
pub fn declspec(mut token: TokenWrap) -> (TokenWrap, Option<*mut Ty>) {
    token.set(skip(token, "int"));
    return (token, create_ty(TypeKind::INT));
}

#[allow(dead_code)]
pub fn declarator(mut token: TokenWrap, mut ty: Option<*mut Ty>) -> (Option<*mut Ty>, TokenWrap) {
    while consume(token, "*").0 {
        token = consume(token, "*").1;
        ty = Some(Box::leak(Box::new(Ty::point_to(ty))));
    }

    if token.kind() != TokenKind::IDENT {
        error_token(token, "expected a variable name");
    }

    let start = token;

    let (typ, tk) = ty_suffix(token.reset_by_next(), ty);
    ty = typ;
    set_ty_token(ty, start);

    return (ty, tk);
}

#[allow(dead_code)]
pub fn declaration(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let base_ty = declspec(token).1;
    token = declspec(token).0;

    let head: Option<*mut Node> = create_node(NodeKind::Num, token);
    let mut cur = head;

    let mut i = 0;
    while !equal(token, ";") {
        if i > 0 {
            token.set(skip(token, ","));
        }
        i += 1;

        let ty = declarator(token, base_ty).0;
        token = declarator(token, base_ty).1;
        let var = Obj::new(get_ident(get_ty_token(ty)), ty);

        if !equal(token, "=") {
            continue;
        }

        let lhs = create_var_node(Some(var), get_ty_token(ty));
        let rhs = assign(token.reset_by_next());
        token = rhs.1;
        let node = create_binary_node(NodeKind::ASSIGN, lhs, rhs.0, token);

        set_node_next(cur, create_unary_node(NodeKind::ExprStmt, node, token));
        cur = get_node_next(cur);
    }

    let node = create_node(NodeKind::BLOCK, token);
    set_node_body(node, get_node_next(head));
    token.reset_by_next();

    return (node, token);
}

#[allow(dead_code)]
pub fn func_call(mut token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let start = token;
    token.reset_by_next();
    token.reset_by_next();

    let head: Option<*mut Node> = create_node(NodeKind::Num, token);
    let mut cur = head;

    while !equal(token, ")") {
        if cur != head {
            token.set(skip(token, ","));
        }
        let (n, t) = assign(token);
        set_node_next(cur, n);
        cur = get_node_next(cur);
        token = t;
    }
    token.set(skip(token, ")"));

    let node = create_node(NodeKind::FUNCALL, start);
    let len = start.get_len();
    let func_name: String = start.get_loc().unwrap()[..len].iter().collect();
    set_node_func_name(node, Box::leak(Box::new(func_name)));

    set_node_args(node, get_node_next(head));

    return (node, token);
}

#[allow(dead_code)]
pub fn func_params(mut token: TokenWrap, mut ty: Option<*mut Ty>) -> (Option<*mut Ty>, TokenWrap) {
    let head: Option<*mut Ty> = Some(Box::leak(Box::new(Ty::new())));
    let mut cur = head;

    while !equal(token, ")") {
        if cur != head {
            token.set(skip(token, ","));
        }
        let (tk, base_ty) = declspec(token);
        let (declar_ty, tk) = declarator(tk, base_ty);

        set_ty_next(
            cur,
            Ty::copy(unsafe { declar_ty.unwrap().as_ref().unwrap() }),
        );
        cur = get_ty_next(cur);
        token = tk;
    }

    ty = Some(Box::leak(Box::new(Ty::new_func_ty(ty))));
    set_ty_params(ty, get_ty_next(head));

    token.reset_by_next();

    return (ty, token);
}

#[allow(dead_code)]
pub fn ty_suffix(mut token: TokenWrap, ty: Option<*mut Ty>) -> (Option<*mut Ty>, TokenWrap) {
    if equal(token, "(") {
        let mut start = token;
        return func_params(start.set(start.next()), ty);
    }

    if equal(token, "[") {
        let mut start = token;
        let sz = get_number(start.reset_by_next());
        token.reset_by_next();
        token.reset_by_next();
        token.set(skip(token, "]"));
        let (ty, token) = ty_suffix(token, ty);
        let ty: Option<*mut Ty> = Some(Box::leak(Box::new(Ty::new_array_ty(ty, sz as usize))));
        return (ty, token);
    }

    return (ty, token);
}

#[allow(dead_code)]
pub fn function(mut token: TokenWrap) -> (Option<*mut Function>, TokenWrap) {
    let (typ, tk) = declspec(token);
    let (typ, tk) = declarator(typ, tk);

    unsafe { LOCALS = None };

    let mut func = Function::empty();
    func.name = get_ident(get_ty_token(typ));

    create_param_l_vars(get_ty_params(typ));
    func.params = unsafe { LOCALS };

    token.set(skip(tk, "{"));
    let (n, t) = compound_stmt(token);
    func.body = n;
    func.locals = unsafe { LOCALS };

    return (Some(Box::leak(Box::new(func))), t);
}

#[allow(dead_code)]
pub fn parse(mut token: TokenWrap) -> Option<*mut Function> {
    let head: Option<*mut Function> = Some(Box::leak(Box::new(Function::empty())));
    let mut cur = head;

    while token.kind() != TokenKind::EOF {
        let (f, tk) = function(token);
        set_function_next(cur, f);
        cur = get_function_next(cur);
        token = tk;
    }

    return get_function_next(head);
}

#[allow(dead_code)]
pub fn create_param_l_vars(params: Option<*mut Ty>) {
    if !params.is_none() {
        create_param_l_vars(get_ty_next(params));
        Obj::new(get_ident(get_ty_token(params)), params);
    }
}

#[allow(dead_code)]
pub fn get_number(token: TokenWrap) -> i32 {
    if token.kind() != TokenKind::Num {
        error_token(token, "expected a number");
    }
    return token.val();
}

#[allow(dead_code)]
pub fn postfix(token: TokenWrap) -> (Option<*mut Node>, TokenWrap) {
    let (mut node, mut token) = primary(token);

    while equal(token, "[") {
        let start = token;
        let (idx, mut tk) = expr(token.reset_by_next());
        tk.set(skip(tk, "]"));
        token = tk;

        let (nd, _) = new_add(node, idx, start);
        let nd = create_unary_node(NodeKind::DEREF, nd, start);
        node = nd;
    }
    return (node, token);
}

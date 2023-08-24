use crate::{
    rvcc::{ NodeKind, NodeWrap, ObjWrap, TokenKind, TokenWrap, TyWrap, TypeKind, FunctionWrap},
    tokenize::{consume, equal, skip},
    ty::{add_ty, is_int},
    utils::error_token,
};

pub static mut LOCALS: ObjWrap = ObjWrap::empty();

#[allow(dead_code)]
pub fn expr_v2(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    return assign_v2(token);
}

#[allow(dead_code)]
pub fn assign_v2(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = equality_v2(token);
    if equal(token, "=") {
        let (n, t) = assign_v2(token.reset_by_next());
        node = NodeWrap::new_binary(NodeKind::ASSIGN, node, n, token);
        token = t;
    }

    return (node, token);
}

#[allow(dead_code)]
fn equality_v2(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = relational_v2(token);

    loop {
        if equal(token, "==") {
            let (n, t) = relational_v2(token.reset_by_next());
            node = NodeWrap::new_binary(NodeKind::EQ, node, n, t);
            token = t;
            continue;
        }
        if equal(token, "!=") {
            let (n, t) = relational_v2(token.reset_by_next());
            node = NodeWrap::new_binary(NodeKind::NE, node, n, t);
            token = t;
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
fn relational_v2(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = add_v2(token);

    loop {
        if equal(token, "<") {
            let (n, t) = add_v2(token.reset_by_next());
            node = NodeWrap::new_binary(NodeKind::LT, node, n, t);
            token = t;
            continue;
        }

        if equal(token, "<=") {
            let (n, t) = add_v2(token.reset_by_next());
            node = NodeWrap::new_binary(NodeKind::LE, node, n, t);
            token = t;
            continue;
        }

        if equal(token, ">") {
            let (n, t) = add_v2(token.reset_by_next());
            node = NodeWrap::new_binary(NodeKind::LT, n, node, t);
            token = t;
            continue;
        }

        if equal(token, ">=") {
            let (n, t) = add_v2(token.reset_by_next());
            node = NodeWrap::new_binary(NodeKind::LE, n, node, t);
            token = t;
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
pub fn new_add_v2(mut lhs: NodeWrap, mut rhs: NodeWrap, token: TokenWrap) -> (NodeWrap, TokenWrap) {
    add_ty(lhs);
    add_ty(rhs);

    if is_int(lhs.ty()) && is_int(rhs.ty()) {
        let node = NodeWrap::new_binary(NodeKind::Add, lhs, rhs, token);
        return (node, token);
    }
    if !lhs.ty().base().ptr.is_none() && !rhs.ty().base().ptr.is_none() {
        error_token(token, "invalid operands")
    }
    if lhs.ty().base().ptr.is_none() && !rhs.ty().base().ptr.is_none() {
        let tmp = lhs;
        lhs = rhs;
        rhs = tmp;
    }
    let val = lhs.ty().base().size();
    let num_node = NodeWrap::new_num(val as i64, token);
    let rhs = NodeWrap::new_binary(NodeKind::Mul, rhs, num_node, token);
    let node = NodeWrap::new_binary(NodeKind::Add, lhs, rhs, token);
    return (node, token);
}

#[allow(dead_code)]
pub fn new_sub_v2(lhs: NodeWrap, rhs: NodeWrap, token: TokenWrap) -> (NodeWrap, TokenWrap) {
    add_ty(lhs);
    add_ty(rhs);

    if is_int(lhs.ty()) && is_int(rhs.ty()) {
        let node = NodeWrap::new_binary(NodeKind::Sub, lhs, rhs, token);
        return (node, token);
    }

    if !((lhs.ty().base().ptr).is_none()) && is_int(rhs.ty()) {
        let val = lhs.ty().base().size();
        let num_node = NodeWrap::new_num(val as i64, token);
        let rhs_node = NodeWrap::new_binary(NodeKind::Mul, rhs, num_node, token);
        add_ty(rhs_node);
        let node = NodeWrap::new_binary(NodeKind::Sub, lhs, rhs_node, token);
        node.set_ty(lhs.ty());
        return (node, token);
    }
    if !lhs.ty().base().ptr.is_none() && !rhs.ty().base().ptr.is_none() {
        let node = NodeWrap::new_binary(NodeKind::Sub, lhs, rhs, token);
        let ty = TyWrap::new_with_kind(Some(TypeKind::INT));
        node.set_ty(ty);
        let val = lhs.ty().base().size();
        let num_node = NodeWrap::new_num(val as i64, token);
        let node = NodeWrap::new_binary(NodeKind::Div, node, num_node, token);
        return (node, token);
    }
    error_token(token, "invalid operands");
    return (NodeWrap::empty(), token);
}

#[allow(dead_code)]
fn add_v2(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = mul_v2(token);

    loop {
        if equal(token, "+") {
            let start = token;
            let (n, t) = mul_v2(token.reset_by_next());
            let (n, _t) = new_add_v2(node, n, start);
            node = n;
            token = t;
            continue;
        }
        if equal(token, "-") {
            let start = token;
            let (n, t) = mul_v2(token.reset_by_next());
            let (n, _t) = new_sub_v2(node, n, start);
            node = n;
            token = t;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn mul_v2(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = unary_v2(token);

    loop {
        let start = token;
        if equal(token, "*") {
            let (n, t) = unary_v2(token.reset_by_next());
            node = NodeWrap::new_binary(NodeKind::Mul, node, n, start);
            token = t;
            continue;
        }
        if equal(token, "/") {
            let (n, t) = unary_v2(token.reset_by_next());
            node = NodeWrap::new_binary(NodeKind::Div, node, n, start);
            token = t;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn unary_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "+") {
        return unary_v2(token.reset_by_next());
    }
    if equal(token, "-") {
        let (n, t) = unary_v2(token.reset_by_next());
        return (NodeWrap::new_unary(NodeKind::NEG, n, t), t);
    }
    if equal(token, "&") {
        let (n, t) = unary_v2(token.reset_by_next());
        return (NodeWrap::new_unary(NodeKind::ADDR, n, t), t);
    }
    if equal(token, "*") {
        let (n, t) = unary_v2(token.reset_by_next());
        return (NodeWrap::new_unary(NodeKind::DEREF, n, t), t);
    }

    postfix_v2(token)
}

#[allow(dead_code)]
fn primary_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "(") {
        let (n, t) = expr_v2(token.reset_by_next());
        token = t;
        return (n, token.set(skip(token, ")")));
    }

    if token.kind() == TokenKind::IDENT {
        let mut start = token;
        if equal(start.reset_by_next(), "(") {
            return func_call_v2(token);
        }

        let var = find_var(token);
        if var.ptr.is_none() {
            error_token(token, "undefined variable");
        }
        let node = NodeWrap::new_var_node(var, token);
        return (node, token.reset_by_next());
    }

    if token.kind() == TokenKind::Num {
        let node = NodeWrap::new_num(token.val() as i64, token);
        token.reset_by_next();
        return (node, token);
    }

    error_token(token, "expected an expression");
    (NodeWrap::empty(), token)
}

#[allow(dead_code)]
pub fn compound_stmt_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let head = NodeWrap::new(NodeKind::Num, token);
    let mut cur = head;

    while !equal(token, "}") {
        if equal(token, "int") {
            let dec = declaration_v2(token);
            token = dec.1;
            cur.set_next(dec.0)
        } else {
            let (n, t) = stmt_v2(token);
            token = t;
            cur.set_next(n)
        }

        cur = cur.next();
        add_ty(cur);
    }

    let node = NodeWrap::new(NodeKind::BLOCK, token);
    node.set_body(head.next());
    return (node, token.reset_by_next());
}

#[allow(dead_code)]
fn stmt_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "return") {
        let node = NodeWrap::new(NodeKind::RETURN, token);
        let (n, t) = expr_v2(token.reset_by_next());
        node.set_lhs(n);

        token.set(skip(t, ";"));
        return (node, token);
    }

    if equal(token, "if") {
        let node = NodeWrap::new(NodeKind::IF, token);

        token.reset_by_next();
        token.set(skip(token, "("));

        let (n, t) = expr_v2(token);
        node.set_cond(n);

        token.set(skip(t, ")"));
        let (n, t) = stmt_v2(token);
        token = t;
        node.set_then(n);

        if equal(token, "else") {
            token.reset_by_next();
            let (n, t) = stmt_v2(token);
            node.set_els(n);
            token = t;
        }
        return (node, token);
    }

    if equal(token, "for") {
        let node = NodeWrap::new(NodeKind::FOR, token);

        token.reset_by_next();
        token.set(skip(token, "("));

        let (n, mut token) = expr_stmt_v2(token);
        node.set_init(n);

        if !equal(token, ";") {
            let (n, t) = expr_v2(token);
            node.set_cond(n);
            token = t;
        }
        token.set(skip(token, ";"));

        if !equal(token, ")") {
            let (n, t) = expr_v2(token);
            node.set_inc(n);
            token = t;
        }
        token.set(skip(token, ")"));

        let (n, token) = stmt_v2(token);
        node.set_then(n);

        return (node, token);
    }

    if equal(token, "while") {
        let node = NodeWrap::new(NodeKind::FOR, token);

        token.reset_by_next();
        token.set(skip(token, "("));

        let (n, mut token) = expr_v2(token);
        node.set_cond(n);
        token.set(skip(token, ")"));

        let (n, token) = stmt_v2(token);
        node.set_then(n);

        return (node, token);
    }

    if equal(token, "{") {
        return compound_stmt_v2(token.reset_by_next());
    }
    expr_stmt_v2(token)
}

#[allow(dead_code)]
fn expr_stmt_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, ";") {
        token.reset_by_next();
        return (NodeWrap::new(NodeKind::BLOCK, token), token);
    }

    let (n, t) = expr_v2(token);
    let node = NodeWrap::new_unary(NodeKind::ExprStmt, n, token);
    token.set(skip(t, ";"));
    return (node, token);
}

#[allow(dead_code)]
pub fn find_var(token: TokenWrap) -> ObjWrap {
    if unsafe { LOCALS.ptr.is_none() } {
        return ObjWrap::empty();
    }
    let mut var = unsafe { LOCALS };
    loop {
        let name = var.name();
        if var.name().len() == token.get_len() && equal(token, name) {
            return var;
        }
        if var.nxt().ptr.is_none() {
            break;
        }
        var = var.nxt();
    }
    ObjWrap::empty()
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
pub fn declspec(mut token: TokenWrap) -> (TokenWrap, TyWrap) {
    token.set(skip(token, "int"));
    return (token, TyWrap::new_with_kind(Some(TypeKind::INT)));
}

#[allow(dead_code)]
pub fn declarator(mut token: TokenWrap, mut ty: TyWrap) -> (TyWrap, TokenWrap) {
    while consume(token, "*").0 {
        token = consume(token, "*").1;
        ty = TyWrap::point_to(ty);
    }

    if token.kind() != TokenKind::IDENT {
        error_token(token, "expected a variable name");
    }

    let start = token;

    let (typ, tk) = ty_suffix(token.reset_by_next(), ty);
    ty = typ;
    ty.set_token(start);

    return (ty, tk);
}

#[allow(dead_code)]
pub fn declaration_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let base_ty = declspec(token).1;
    token = declspec(token).0;

    let head = NodeWrap::new(NodeKind::Num, token);
    let mut cur = head;

    let mut i = 0;
    while !equal(token, ";") {
        if i > 0 {
            token.set(skip(token, ","));
        }
        i += 1;

        let ty = declarator(token, base_ty).0;
        token = declarator(token, base_ty).1;
        let var = ObjWrap::new(get_ident(ty.token()), ty);

        if !equal(token, "=") {
            continue;
        }

        let lhs = NodeWrap::new_var_node(var, ty.token());
        let rhs = assign_v2(token.reset_by_next());
        token = rhs.1;
        let node = NodeWrap::new_binary(NodeKind::ASSIGN, lhs, rhs.0, token);

        cur.set_next(NodeWrap::new_unary(NodeKind::ExprStmt, node, token));
        cur = cur.next();
    }

    let node = NodeWrap::new(NodeKind::BLOCK, token);
    node.set_body(head.next());
    token.reset_by_next();

    return (node, token);
}

#[allow(dead_code)]
pub fn func_call_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let start = token;
    token.reset_by_next();
    token.reset_by_next();

    let head = NodeWrap::new(NodeKind::Num, token);
    let mut cur = head;

    while !equal(token, ")") {
        if cur != head {
            token.set(skip(token, ","));
        }
        let (n, t) = assign_v2(token);
        cur.set_next(n);
        cur = cur.next();
        token = t;
    }
    token.set(skip(token, ")"));

    let node = NodeWrap::new(NodeKind::FUNCALL, start);
    let len = start.get_len();
    let func_name: String = start.get_loc().unwrap()[..len].iter().collect();
    node.set_func_name(Box::leak(Box::new(func_name)));

    node.set_args(head.next());

    return (node, token);
}

#[allow(dead_code)]
pub fn func_params(mut token: TokenWrap, mut ty: TyWrap) -> (TyWrap, TokenWrap) {
    let head = TyWrap::new();
    let mut cur = head;

    while !equal(token, ")") {
        if cur != head {
            token.set(skip(token, ","));
        }
        let (tk, base_ty) = declspec(token);
        let (declar_ty, tk) = declarator(tk, base_ty);

        cur.set_next(TyWrap::copy(declar_ty));
        cur = cur.next();
        token = tk;
    }

    ty = TyWrap::new_func_ty(ty);
    ty.set_params(head.next());

    token.reset_by_next();

    return (ty, token);
}

#[allow(dead_code)]
pub fn ty_suffix(mut token: TokenWrap, ty: TyWrap) -> (TyWrap, TokenWrap) {
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
        let ty = TyWrap::new_array_ty(ty, sz as usize);
        return (ty, token);
    }

    return (ty, token);
}

#[allow(dead_code)]
pub fn function(mut token: TokenWrap) -> (FunctionWrap, TokenWrap) {
    let (typ, tk) = declspec(token);
    let (typ, tk) = declarator(typ, tk);

    unsafe { LOCALS = ObjWrap::empty() };

    let func = FunctionWrap::init();
    func.set_name(get_ident(typ.token()));

    create_param_l_vars(typ.params());
    func.set_params( unsafe { LOCALS });

    token.set(skip(tk, "{"));
    let (n, t) = compound_stmt_v2(token);
    func.set_body(n);
    func.set_locals(unsafe { LOCALS });

    return (func, t);
}

#[allow(dead_code)]
pub fn parse(mut token: TokenWrap) -> FunctionWrap {
    let head = FunctionWrap::init();
    let mut cur = head;

    while token.kind() != TokenKind::EOF {
        let (f, tk) = function(token);
        cur.set_nxt(f);
        cur = cur.nxt();
        token = tk;
    }

    return head.nxt();
}

#[allow(dead_code)]
pub fn create_param_l_vars(params: TyWrap) {
    if !params.ptr.is_none() {
        create_param_l_vars(params.next());
        ObjWrap::new(get_ident(params.token()), params);
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
pub fn postfix_v2(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = primary_v2(token);

    while equal(token, "[") {
        let start = token;
        let (idx, mut tk) = expr_v2(token.reset_by_next());
        tk.set(skip(tk, "]"));
        token = tk;

        let (nd, _) = new_add_v2(node, idx, start);
        let nd = NodeWrap::new_unary(NodeKind::DEREF, nd, start);
        node = nd;
    }
    return (node, token);
}

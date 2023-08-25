use crate::{
    function::FunctionWrap,
    node::{NodeKind::*, NodeWrap},
    obj::ObjWrap,
    token::{consume, equal, skip, TokenKind, TokenWrap},
    ty::{add_ty, is_int, TyWrap, TypeKind},
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
        let (n, t) = assign_v2(token.nxt());
        node = NodeWrap::new_binary(ASSIGN, node, n, token);
        token = t;
    }

    return (node, token);
}

#[allow(dead_code)]
fn equality_v2(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = relational_v2(token);

    loop {
        if equal(token, "==") {
            let (nd, tk) = relational_v2(token.nxt());
            node = NodeWrap::new_binary(EQ, node, nd, tk);
            token = tk;
            continue;
        }
        if equal(token, "!=") {
            let (nd, tk) = relational_v2(token.nxt());
            node = NodeWrap::new_binary(NE, node, nd, tk);
            token = tk;
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
            let (nd, tk) = add_v2(token.nxt());
            node = NodeWrap::new_binary(LT, node, nd, tk);
            token = tk;
            continue;
        }

        if equal(token, "<=") {
            let (nd, tk) = add_v2(token.nxt());
            node = NodeWrap::new_binary(LE, node, nd, tk);
            token = tk;
            continue;
        }

        if equal(token, ">") {
            let (nd, tk) = add_v2(token.nxt());
            node = NodeWrap::new_binary(LT, nd, node, tk);
            token = tk;
            continue;
        }

        if equal(token, ">=") {
            let (nd, tk) = add_v2(token.nxt());
            node = NodeWrap::new_binary(LE, nd, node, tk);
            token = tk;
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
        let node = NodeWrap::new_binary(Add, lhs, rhs, token);
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
    let rhs = NodeWrap::new_binary(Mul, rhs, num_node, token);
    let node = NodeWrap::new_binary(Add, lhs, rhs, token);
    return (node, token);
}

#[allow(dead_code)]
pub fn new_sub_v2(lhs: NodeWrap, rhs: NodeWrap, token: TokenWrap) -> (NodeWrap, TokenWrap) {
    add_ty(lhs);
    add_ty(rhs);

    if is_int(lhs.ty()) && is_int(rhs.ty()) {
        let node = NodeWrap::new_binary(Sub, lhs, rhs, token);
        return (node, token);
    }

    if !((lhs.ty().base().ptr).is_none()) && is_int(rhs.ty()) {
        let val = lhs.ty().base().size();
        let num_node = NodeWrap::new_num(val as i64, token);
        let rhs_node = NodeWrap::new_binary(Mul, rhs, num_node, token);
        add_ty(rhs_node);
        let node = NodeWrap::new_binary(Sub, lhs, rhs_node, token);
        node.set_ty(lhs.ty());
        return (node, token);
    }
    if !lhs.ty().base().ptr.is_none() && !rhs.ty().base().ptr.is_none() {
        let node = NodeWrap::new_binary(Sub, lhs, rhs, token);
        let ty = TyWrap::new_with_kind(Some(TypeKind::INT));
        node.set_ty(ty);
        let val = lhs.ty().base().size();
        let num_node = NodeWrap::new_num(val as i64, token);
        let node = NodeWrap::new_binary(Div, node, num_node, token);
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
            let (nd, tk) = mul_v2(token.nxt());
            let (nd, _t) = new_add_v2(node, nd, token);
            node = nd;
            token = tk;
            continue;
        }
        if equal(token, "-") {
            let (nd, tk) = mul_v2(token.nxt());
            let (nd, _t) = new_sub_v2(node, nd, token);
            node = nd;
            token = tk;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn mul_v2(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = unary_v2(token);

    loop {
        if equal(token, "*") {
            let (nd, tk) = unary_v2(token.nxt());
            node = NodeWrap::new_binary(Mul, node, nd, token);
            token = tk;
            continue;
        }
        if equal(token, "/") {
            let (nd, tk) = unary_v2(token.nxt());
            node = NodeWrap::new_binary(Div, node, nd, token);
            token = tk;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn unary_v2(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "+") {
        return unary_v2(token.nxt());
    }
    if equal(token, "-") {
        let (nd, tk) = unary_v2(token.nxt());
        return (NodeWrap::new_unary(NEG, nd, tk), tk);
    }
    if equal(token, "&") {
        let (nd, tk) = unary_v2(token.nxt());
        return (NodeWrap::new_unary(ADDR, nd, tk), tk);
    }
    if equal(token, "*") {
        let (nd, tk) = unary_v2(token.nxt());
        return (NodeWrap::new_unary(DEREF, nd, tk), tk);
    }

    postfix_v2(token)
}

#[allow(dead_code)]
fn primary_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "(") {
        let (nd, tk) = expr_v2(token.nxt());
        token = skip(tk, ")");
        return (nd, token);
    }

    if token.kind() == TokenKind::IDENT {
        if equal(token.nxt(), "(") {
            return func_call_v2(token);
        }

        let var = find_var(token);
        if var.ptr.is_none() {
            error_token(token, "undefined variable");
        }
        let node = NodeWrap::new_var_node(var, token);
        return (node, token.nxt());
    }

    if token.kind() == TokenKind::Num {
        let node = NodeWrap::new_num(token.val() as i64, token);
        token = token.nxt();
        return (node, token);
    }

    error_token(token, "expected an expression");
    (NodeWrap::empty(), token)
}

#[allow(dead_code)]
pub fn compound_stmt_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let head = NodeWrap::new(Num, token);
    let mut cur = head;

    while !equal(token, "}") {
        if equal(token, "int") {
            let (nd, tk) = declaration_v2(token);
            token = tk;
            cur.set_nxt(nd)
        } else {
            let (nd, tk) = stmt_v2(token);
            token = tk;
            cur.set_nxt(nd)
        }

        cur = cur.nxt();
        add_ty(cur);
    }

    let node = NodeWrap::new(BLOCK, token);
    node.set_body(head.nxt());
    return (node, token.nxt());
}

#[allow(dead_code)]
fn stmt_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "return") {
        let node = NodeWrap::new(RETURN, token);
        let (nd, tk) = expr_v2(token.nxt());
        node.set_lhs(nd);

        token = skip(tk, ";");
        return (node, token);
    }

    if equal(token, "if") {
        let node = NodeWrap::new(IF, token);

        token = skip(token.nxt(), "(");

        let (nd, mut token) = expr_v2(token);
        node.set_cond(nd);

        token = skip(token, ")");
        let (nd, mut token) = stmt_v2(token);
        node.set_then(nd);

        if equal(token, "else") {
            token = token.nxt();
            let (nd, tk) = stmt_v2(token);
            node.set_els(nd);
            token = tk;
        }
        return (node, token);
    }

    if equal(token, "for") {
        let node = NodeWrap::new(FOR, token);

        token = token.nxt();
        token = skip(token, "(");

        let (nd, mut token) = expr_stmt_v2(token);
        node.set_init(nd);

        if !equal(token, ";") {
            let (nd, tk) = expr_v2(token);
            node.set_cond(nd);
            token = tk;
        }
        token = skip(token, ";");

        if !equal(token, ")") {
            let (nd, tk) = expr_v2(token);
            node.set_inc(nd);
            token = tk;
        }
        token = skip(token, ")");

        let (nd, token) = stmt_v2(token);
        node.set_then(nd);

        return (node, token);
    }

    if equal(token, "while") {
        let node = NodeWrap::new(FOR, token);

        token = token.nxt();
        token = skip(token, "(");

        let (nd, mut token) = expr_v2(token);
        node.set_cond(nd);
        token = skip(token, ")");

        let (nd, token) = stmt_v2(token);
        node.set_then(nd);

        return (node, token);
    }

    if equal(token, "{") {
        return compound_stmt_v2(token.nxt());
    }
    expr_stmt_v2(token)
}

#[allow(dead_code)]
fn expr_stmt_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, ";") {
        token = token.nxt();
        return (NodeWrap::new(BLOCK, token), token);
    }

    let (nd, t) = expr_v2(token);
    let node = NodeWrap::new_unary(ExprStmt, nd, token);
    token = skip(t, ";");
    return (node, token);
}

#[allow(dead_code)]
pub fn find_var(token: TokenWrap) -> ObjWrap {
    if unsafe { LOCALS.ptr.is_none() } {
        return ObjWrap::empty();
    }
    let vars = unsafe { LOCALS };
    for var in vars {
        let name = var.name();
        if var.name().len() == token.get_len() && equal(token, name) {
            return var;
        }
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
    token = skip(token, "int");
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

    let (typ, tk) = ty_suffix(token.nxt(), ty);
    ty = typ;
    ty.set_token(token);

    return (ty, tk);
}

#[allow(dead_code)]
pub fn declaration_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let base_ty = declspec(token).1;
    token = declspec(token).0;

    let head = NodeWrap::new(Num, token);
    let mut cur = head;

    let mut i = 0;
    while !equal(token, ";") {
        if i > 0 {
            token = skip(token, ",");
        }
        i += 1;

        let ty = declarator(token, base_ty).0;
        token = declarator(token, base_ty).1;
        let var = ObjWrap::new(get_ident(ty.token()), ty);

        if !equal(token, "=") {
            continue;
        }

        let lhs = NodeWrap::new_var_node(var, ty.token());
        let rhs = assign_v2(token.nxt());
        token = rhs.1;
        let node = NodeWrap::new_binary(ASSIGN, lhs, rhs.0, token);

        cur.set_nxt(NodeWrap::new_unary(ExprStmt, node, token));
        cur = cur.nxt();
    }

    let node = NodeWrap::new(BLOCK, token);
    node.set_body(head.nxt());

    return (node, token.nxt());
}

#[allow(dead_code)]
pub fn func_call_v2(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let start = token;
    token = token.nxt().nxt();

    let head = NodeWrap::new(Num, token);
    let mut cur = head;

    while !equal(token, ")") {
        if cur != head {
            token = skip(token, ",");
        }
        let (n, t) = assign_v2(token);
        cur.set_nxt(n);
        cur = cur.nxt();
        token = t;
    }
    token = skip(token, ")");

    let node = NodeWrap::new(FUNCALL, start);
    let len = start.get_len();
    let func_name: String = start.get_loc().unwrap()[..len].iter().collect();
    node.set_func_name(Box::leak(Box::new(func_name)));

    node.set_args(head.nxt());

    return (node, token);
}

#[allow(dead_code)]
pub fn func_params(mut token: TokenWrap, mut ty: TyWrap) -> (TyWrap, TokenWrap) {
    let head = TyWrap::new();
    let mut cur = head;

    while !equal(token, ")") {
        if cur != head {
            token = skip(token, ",");
        }
        let (tk, base_ty) = declspec(token);
        let (declar_ty, tk) = declarator(tk, base_ty);

        cur.set_next(TyWrap::copy(declar_ty));
        cur = cur.next();
        token = tk;
    }

    ty = TyWrap::new_func_ty(ty);
    ty.set_params(head.next());

    return (ty, token.nxt());
}

#[allow(dead_code)]
pub fn ty_suffix(mut token: TokenWrap, ty: TyWrap) -> (TyWrap, TokenWrap) {
    if equal(token, "(") {
        return func_params(token.nxt(), ty);
    }

    if equal(token, "[") {
        let sz = get_number(token.nxt());
        token = token.nxt().nxt();
        token = skip(token, "]");
        let (ty, token) = ty_suffix(token, ty);
        let ty = TyWrap::new_array_ty(ty, sz as usize);
        return (ty, token);
    }

    return (ty, token);
}

#[allow(dead_code)]
pub fn function(token: TokenWrap) -> (FunctionWrap, TokenWrap) {
    let (typ, token) = declspec(token);
    let (typ, mut token) = declarator(typ, token);

    unsafe { LOCALS = ObjWrap::empty() };

    let func = FunctionWrap::init();
    func.set_name(get_ident(typ.token()));

    create_param_l_vars(typ.params());
    func.set_params(unsafe { LOCALS });

    token = skip(token, "{");
    let (nd, token) = compound_stmt_v2(token);

    func.set_body(nd);
    func.set_locals(unsafe { LOCALS });

    return (func, token);
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
        let (nd, mut tk) = expr_v2(token.nxt());
        tk = skip(tk, "]");

        let (nd, _) = new_add_v2(node, nd, token);
        let nd = NodeWrap::new_unary(DEREF, nd, token);
        node = nd;
        token = tk
    }
    return (node, token);
}

use crate::{
    node::{
        MemberWrap,
        NodeKind::{self, *},
        NodeWrap,
    },
    obj::ObjWrap,
    scope::{ScopeWrap, SCOPE},
    token::{consume, equal, skip, TokenKind, TokenWrap},
    ty::{add_ty, is_int, TyWrap, TypeKind},
    utils::error_token,
};

#[allow(dead_code)]
pub static mut LOCALS: ObjWrap = ObjWrap::empty();
#[allow(dead_code)]
pub static mut GLOBALS: ObjWrap = ObjWrap::empty();
#[allow(dead_code)]
pub static mut VAR_IDXS: usize = 0;


#[allow(dead_code)]
pub fn find_var(token: TokenWrap) -> ObjWrap {
    for sc in unsafe { SCOPE } {
        for vs in sc.vars() {
            if equal(token, vs.name()) {
                return vs.var();
            }
        }
    }
    return ObjWrap::empty();
}

#[allow(dead_code)]
pub fn get_ident(token: TokenWrap) -> &'static str {
    if token.kind() != TokenKind::IDENT {
        error_token(token, "expected an identifier");
    }

    let len = token.len();
    let name: String = token.loc().unwrap()[..len].iter().collect();
    Box::leak(Box::new(name))
}

#[allow(dead_code)]
pub fn get_number(token: TokenWrap) -> i32 {
    if token.kind() != TokenKind::Num {
        error_token(token, "expected a number");
    }
    return token.val();
}

#[allow(dead_code)]
pub fn new_unique_name() -> &'static str {
    let s = Box::leak(Box::new(format!(".L..{}", unsafe { VAR_IDXS })));
    unsafe { VAR_IDXS += 1 };
    return s;
}

#[allow(dead_code)]
pub fn new_anon_g_var(ty: TyWrap) -> ObjWrap {
    ObjWrap::new_global(new_unique_name(), ty)
}

#[allow(dead_code)]
pub fn new_string_literal(stri: Vec<usize>, ty: TyWrap) -> ObjWrap {
    let var = new_anon_g_var(ty);
    var.set_init_data(stri);
    return var;
}

#[allow(dead_code)]
pub fn declspec(token: TokenWrap) -> (TokenWrap, TyWrap) {
    if equal(token, "char") {
        return (token.nxt(), TyWrap::new_with_kind(Some(TypeKind::CHAR)));
    }

    if equal(token, "int") {
        return (token.nxt(), TyWrap::new_with_kind(Some(TypeKind::INT)));
    }

    if equal(token, "struct") {
        return struct_decl(token.nxt());
    }

    error_token(token, "typename expected");
    return (TokenWrap::empty(), TyWrap::empty());
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

        cur.set_next(declar_ty);
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
        token = skip(token.nxt().nxt(), "]");
        let (ty, token) = ty_suffix(token, ty);
        let ty = TyWrap::new_array_ty(ty, sz as usize);
        return (ty, token);
    }

    return (ty, token);
}

#[allow(dead_code)]
pub fn declarator(mut token: TokenWrap, mut ty: TyWrap) -> (TyWrap, TokenWrap) {
    while consume(&mut token, "*") {
        ty = TyWrap::point_to(ty);
    }

    if token.kind() != TokenKind::IDENT {
        error_token(token, "expected a variable name");
    }

    let (ty, tk) = ty_suffix(token.nxt(), ty);
    ty.set_token(token);

    return (ty, tk);
}

#[allow(dead_code)]
pub fn declaration(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
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
        let var = ObjWrap::new_local(get_ident(ty.name()), ty);

        if !equal(token, "=") {
            continue;
        }

        let lhs = NodeWrap::new_var_node(var, ty.name());
        let rhs = assign(token.nxt());
        token = rhs.1;
        let node = NodeWrap::new_binary(ASSIGN, lhs, rhs.0, token);

        cur.set_nxt(NodeWrap::new_unary(EXPRSTMT, node, token));
        cur = cur.nxt();
    }

    let node = NodeWrap::new(BLOCK, token);
    node.set_body(head.nxt());

    return (node, token.nxt());
}

#[allow(dead_code)]
pub fn is_type_name(token: TokenWrap) -> bool {
    return equal(token, "char") || equal(token, "int") || equal(token, "struct");
}

#[allow(dead_code)]
fn stmt(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "return") {
        let node = NodeWrap::new(RETURN, token);
        let (nd, tk) = expr(token.nxt());
        node.set_lhs(nd);

        token = skip(tk, ";");
        return (node, token);
    }

    if equal(token, "if") {
        let node = NodeWrap::new(IF, token);

        token = skip(token.nxt(), "(");

        let (nd, mut token) = expr(token);
        node.set_cond(nd);

        token = skip(token, ")");
        let (nd, mut token) = stmt(token);
        node.set_then(nd);

        if equal(token, "else") {
            token = token.nxt();
            let (nd, tk) = stmt(token);
            node.set_els(nd);
            token = tk;
        }
        return (node, token);
    }

    if equal(token, "for") {
        let node = NodeWrap::new(FOR, token);

        token = token.nxt();
        token = skip(token, "(");

        let (nd, mut token) = expr_stmt(token);
        node.set_init(nd);

        if !equal(token, ";") {
            let (nd, tk) = expr(token);
            node.set_cond(nd);
            token = tk;
        }
        token = skip(token, ";");

        if !equal(token, ")") {
            let (nd, tk) = expr(token);
            node.set_inc(nd);
            token = tk;
        }
        token = skip(token, ")");

        let (nd, token) = stmt(token);
        node.set_then(nd);

        return (node, token);
    }

    if equal(token, "while") {
        let node = NodeWrap::new(FOR, token);

        token = token.nxt();
        token = skip(token, "(");

        let (nd, mut token) = expr(token);
        node.set_cond(nd);
        token = skip(token, ")");

        let (nd, token) = stmt(token);
        node.set_then(nd);

        return (node, token);
    }

    if equal(token, "{") {
        return compound_stmt(token.nxt());
    }
    expr_stmt(token)
}

#[allow(dead_code)]
pub fn compound_stmt(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let node = NodeWrap::new(BLOCK, token);
    let head = NodeWrap::new(Num, token);
    let mut cur = head;

    let sc = ScopeWrap::new();
    sc.enter();

    while !equal(token, "}") {
        if is_type_name(token) {
            let (nd, tk) = declaration(token);
            token = tk;
            cur.set_nxt(nd)
        } else {
            let (nd, tk) = stmt(token);
            token = tk;
            cur.set_nxt(nd);
        }

        cur = cur.nxt();
        add_ty(cur);
    }
    unsafe { SCOPE.leave() };

    node.set_body(head.nxt());
    return (node, token.nxt());
}

#[allow(dead_code)]
fn expr_stmt(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, ";") {
        return (NodeWrap::new(BLOCK, token.nxt()), token.nxt());
    }

    let node = NodeWrap::new(EXPRSTMT, token);
    let (nd, t) = expr(token);
    node.set_lhs(nd);

    token = skip(t, ";");
    return (node, token);
}

#[allow(dead_code)]
pub fn expr(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (nd, token) = assign(token);

    if equal(token, ",") {
        let (rhs, token) = expr(token.nxt());
        let nd = NodeWrap::new_binary(NodeKind::COMMA, nd, rhs, token);
        return (nd, token);
    }
    return (nd, token);
}

#[allow(dead_code)]
pub fn assign(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = equality(token);
    if equal(token, "=") {
        let (n, t) = assign(token.nxt());
        node = NodeWrap::new_binary(ASSIGN, node, n, token);
        token = t;
    }

    return (node, token);
}

#[allow(dead_code)]
fn equality(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = relational(token);

    loop {
        if equal(token, "==") {
            let (nd, tk) = relational(token.nxt());
            node = NodeWrap::new_binary(EQ, node, nd, tk);
            token = tk;
            continue;
        }
        if equal(token, "!=") {
            let (nd, tk) = relational(token.nxt());
            node = NodeWrap::new_binary(NE, node, nd, tk);
            token = tk;
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
fn relational(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = add(token);

    loop {
        if equal(token, "<") {
            let (nd, tk) = add(token.nxt());
            node = NodeWrap::new_binary(LT, node, nd, tk);
            token = tk;
            continue;
        }

        if equal(token, "<=") {
            let (nd, tk) = add(token.nxt());
            node = NodeWrap::new_binary(LE, node, nd, tk);
            token = tk;
            continue;
        }

        if equal(token, ">") {
            let (nd, tk) = add(token.nxt());
            node = NodeWrap::new_binary(LT, nd, node, tk);
            token = tk;
            continue;
        }

        if equal(token, ">=") {
            let (nd, tk) = add(token.nxt());
            node = NodeWrap::new_binary(LE, nd, node, tk);
            token = tk;
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
pub fn new_add(mut lhs: NodeWrap, mut rhs: NodeWrap, token: TokenWrap) -> (NodeWrap, TokenWrap) {
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
pub fn new_sub(lhs: NodeWrap, rhs: NodeWrap, token: TokenWrap) -> (NodeWrap, TokenWrap) {
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
fn add(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = mul(token);

    loop {
        if equal(token, "+") {
            let (nd, tk) = mul(token.nxt());
            let (nd, _t) = new_add(node, nd, token);
            node = nd;
            token = tk;
            continue;
        }
        if equal(token, "-") {
            let (nd, tk) = mul(token.nxt());
            let (nd, _t) = new_sub(node, nd, token);
            node = nd;
            token = tk;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn mul(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = unary(token);

    loop {
        if equal(token, "*") {
            let (nd, tk) = unary(token.nxt());
            node = NodeWrap::new_binary(Mul, node, nd, token);
            token = tk;
            continue;
        }
        if equal(token, "/") {
            let (nd, tk) = unary(token.nxt());
            node = NodeWrap::new_binary(Div, node, nd, token);
            token = tk;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn unary(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "+") {
        return unary(token.nxt());
    }
    if equal(token, "-") {
        let (nd, tk) = unary(token.nxt());
        return (NodeWrap::new_unary(NEG, nd, tk), tk);
    }
    if equal(token, "&") {
        let (nd, tk) = unary(token.nxt());
        return (NodeWrap::new_unary(ADDR, nd, tk), tk);
    }
    if equal(token, "*") {
        let (nd, tk) = unary(token.nxt());
        return (NodeWrap::new_unary(DEREF, nd, tk), tk);
    }

    postfix(token)
}

#[allow(dead_code)]
pub fn struct_members(mut token: TokenWrap, ty: TyWrap) -> TokenWrap {
    let head = MemberWrap::new();
    let mut cur = head;

    while !equal(token, "}") {
        let (mut tk, base_ty) = declspec(token);
        let mut first = true;

        while !consume(&mut tk, ";") {
            if !first {
                tk = skip(tk, ",");
            }
            first = false;

            let mem = MemberWrap::new();

            let (ty, t) = declarator(tk, base_ty);
            mem.set_ty(ty);
            mem.set_name(mem.ty().name());

            cur.set_next(mem);
            cur = cur.nxt();
            tk = t;
        }
        token = tk;
    }
    ty.set_mems(head.nxt());

    return token.nxt();
}

#[allow(dead_code)]
pub fn struct_decl(mut token: TokenWrap) -> (TokenWrap, TyWrap) {
    token = skip(token, "{");

    let ty = TyWrap::new();
    ty.set_kind(Some(TypeKind::STRUCT));

    token = struct_members(token, ty);

    let mut offset = 0;
    for mem in ty.mems() {
        mem.set_offset(offset);
        offset += mem.ty().size() as i32;
    }

    ty.set_size(offset as usize);

    return (token, ty);
}

#[allow(dead_code)]
pub fn get_struct_member(ty: TyWrap, token: TokenWrap) -> MemberWrap {
    for mem in ty.mems() {
        let len = token.len();
        if mem.name().len() == token.len() && (&mem.name().loc().unwrap()[..len] == &token.loc().unwrap()[..len]) {
            return mem;
        }
    }
    error_token(token, "no such member");
    return MemberWrap::empty();
}

#[allow(dead_code)]
pub fn struct_ref(lhs: NodeWrap, token: TokenWrap) -> NodeWrap {
    add_ty(lhs);

    if lhs.ty().kind() != Some(TypeKind::STRUCT) {
        error_token(lhs.token(), "not a struct");
    }

    let node = NodeWrap::new_unary(NodeKind::MEMBER, lhs, token);
    node.set_mem(get_struct_member(lhs.ty(), token));

    return node;
}

#[allow(dead_code)]
pub fn postfix(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = primary(token);

    loop {
        if equal(token, "[") {
            let (nd, mut tk) = expr(token.nxt());
            tk = skip(tk, "]");

            let (nd, _) = new_add(node, nd, token);
            let nd = NodeWrap::new_unary(DEREF, nd, token);
            node = nd;
            token = tk;
            continue;
        }

        if equal(token, ".") {
            node = struct_ref(node, token.nxt());
            token = token.nxt().nxt();
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
pub fn func_call(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let start = token;
    token = token.nxt().nxt();

    let head = NodeWrap::new(Num, token);
    let mut cur = head;

    while !equal(token, ")") {
        if cur != head {
            token = skip(token, ",");
        }
        let (n, t) = assign(token);
        cur.set_nxt(n);
        cur = cur.nxt();
        token = t;
    }
    token = skip(token, ")");

    let node = NodeWrap::new(FUNCALL, start);
    let len = start.len();
    let func_name: String = start.loc().unwrap()[..len].iter().collect();
    node.set_func_name(Box::leak(Box::new(func_name)));

    node.set_args(head.nxt());

    return (node, token);
}

#[allow(dead_code)]
fn primary(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "(") && equal(token.nxt(), "{") {
        let node = NodeWrap::new(NodeKind::STMTEXPR, token);
        let (nd, tk) = compound_stmt(token.nxt().nxt());
        node.set_body(nd.body());
        token = skip(tk, ")");
        return (node, token);
    }

    if equal(token, "(") {
        let (nd, tk) = expr(token.nxt());
        token = skip(tk, ")");
        return (nd, token);
    }

    if equal(token, "sizeof") {
        let (nd, tk) = unary(token.nxt());
        add_ty(nd);
        return (NodeWrap::new_num(nd.ty().size() as i64, tk), tk);
    }

    if token.kind() == TokenKind::IDENT {
        if equal(token.nxt(), "(") {
            return func_call(token);
        }

        let var = find_var(token);
        if var.ptr.is_none() {
            error_token(token, "undefined variable");
        }
        let node = NodeWrap::new_var_node(var, token);
        return (node, token.nxt());
    }

    if token.kind() == TokenKind::STR {
        let var = new_string_literal(token.stri(), token.ty());
        token = token.nxt();
        return (NodeWrap::new_var_node(var, token), token);
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
pub fn create_param_l_vars(params: TyWrap) {
    if !params.ptr.is_none() {
        create_param_l_vars(params.next());
        ObjWrap::new_local(get_ident(params.name()), params);
    }
}

#[allow(dead_code)]
pub fn function(token: TokenWrap, base_ty: TyWrap) -> (ObjWrap, TokenWrap) {
    let (typ, mut token) = declarator(token, base_ty);

    let func = ObjWrap::new_global(get_ident(typ.name()), typ);
    func.set_is_function(true);
    unsafe { LOCALS = ObjWrap::empty() };

    let sc = ScopeWrap::new();
    sc.enter();

    create_param_l_vars(typ.params());
    func.set_params(unsafe { LOCALS });

    token = skip(token, "{");
    let (nd, token) = compound_stmt(token);

    func.set_body(nd);
    func.set_locals(unsafe { LOCALS });

    unsafe { SCOPE.leave() };

    return (func, token);
}

#[allow(dead_code)]
pub fn global_variable(mut token: TokenWrap, base_ty: TyWrap) -> TokenWrap {
    let mut first = true;

    while !consume(&mut token, ";") {
        if !first {
            token = skip(token, ",");
        }
        first = false;
        let (ty, tk) = declarator(token, base_ty);
        ObjWrap::new_global(get_ident(ty.name()), ty);
        token = tk;
    }
    return token;
}

#[allow(dead_code)]
pub fn is_function(token: TokenWrap) -> bool {
    if equal(token, ";") {
        return false;
    }

    let dummy = TyWrap::new();
    let (ty, _) = declarator(token, dummy);
    return ty.kind() == Some(TypeKind::FUNC);
}

#[allow(dead_code)]
pub fn parse(mut token: TokenWrap) -> ObjWrap {
    unsafe { SCOPE = ScopeWrap::new() }
    unsafe { GLOBALS = ObjWrap::empty() };

    while token.kind() != TokenKind::EOF {
        let (tk, base_ty) = declspec(token);
        if is_function(tk) {
            let (_, tk) = function(tk, base_ty);
            token = tk;
            continue;
        }
        token = global_variable(tk, base_ty);
    }

    return unsafe { GLOBALS };
}


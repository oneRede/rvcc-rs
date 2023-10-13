use crate::{
    codegen::align_to,
    node::{
        MemberWrap,
        NodeKind::{self, *},
        NodeWrap,
    },
    obj::ObjWrap,
    scope::{ScopeWrap, TagScopeWrap, VarAttr, VarScopeWrap, SCOPE},
    token::{consume, equal, skip, TokenKind, TokenWrap},
    ty::{add_ty, is_int, TyWrap, TypeKind},
    utils::error_token,
};

#[allow(dead_code)]
pub static mut LOCALS: ObjWrap = ObjWrap::empty();
#[allow(dead_code)]
pub static mut GLOBALS: ObjWrap = ObjWrap::empty();
#[allow(dead_code)]
pub static mut CURRENT_FN: ObjWrap = ObjWrap::empty();
#[allow(dead_code)]
pub static mut VAR_IDXS: usize = 0;
#[allow(dead_code)]
pub static mut GOTOS: NodeWrap = NodeWrap::empty();
#[allow(dead_code)]
pub static mut LABELS: NodeWrap = NodeWrap::empty();
#[allow(dead_code)]
pub static mut BRAAK_LABELS: &'static str = "";
#[allow(dead_code)]
pub static mut CONT_LABELS: &'static str = "";
#[allow(dead_code)]
pub static mut CURRENT_SWITCH: NodeWrap = NodeWrap::empty();

#[allow(dead_code)]
pub fn find_var(token: TokenWrap) -> VarScopeWrap {
    for sc in unsafe { SCOPE } {
        for vs in sc.vars() {
            if equal(token, vs.name()) {
                return vs;
            }
        }
    }
    return VarScopeWrap::empty();
}

#[allow(dead_code)]
pub fn get_ident(token: TokenWrap) -> &'static str {
    if token.kind() != TokenKind::IDENT {
        error_token(token, "expected an identifier");
    }

    let len = token.len();
    let name = token.loc().unwrap()[..len].iter().collect::<String>();
    Box::leak(Box::new(name))
}

#[allow(dead_code)]
pub fn find_typedef(token: TokenWrap) -> TyWrap {
    if token.kind() == TokenKind::IDENT {
        let vs = find_var(token);
        if !vs.ptr.is_none() {
            return vs.typedef();
        }
    }

    return TyWrap::empty();
}

#[allow(dead_code)]
pub fn get_number(token: TokenWrap) -> i64 {
    if token.kind() != TokenKind::NUM {
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
pub fn find_tag(token: TokenWrap) -> TyWrap {
    for sc in unsafe { SCOPE } {
        for s2 in sc.tags() {
            if equal(token, s2.name()) {
                return s2.ty();
            }
        }
    }
    return TyWrap::empty();
}

#[allow(dead_code)]
pub fn declspec(mut token: TokenWrap, attr: &mut VarAttr) -> (TokenWrap, TyWrap) {
    enum TT {
        VOID = 1 << 0,
        BOOL = 1 << 2,
        CHAR = 1 << 4,
        SHORT = 1 << 6,
        INT = 1 << 8,
        LONG = 1 << 10,
        OTHER = 1 << 12,
    }

    let mut ty = TyWrap::new_with_kind(TypeKind::INT);
    let mut counter: u32 = 0;

    while is_type_name(token) {
        if equal(token, "typedef") || equal(token, "static") {
            if attr.is_typedef.is_none() {
                error_token(
                    token,
                    "storage class specifier is not allowed in this context",
                );
            }
            if equal(token, "typedef") {
                attr.is_typedef = Some(true);
            } else {
                attr.is_static = Some(true);
            }

            if attr.is_typedef.unwrap() && attr.is_static.unwrap() {
                error_token(token, "typedef and static may not be used together");
            }

            token = token.nxt();
            continue;
        }

        let ty2 = find_typedef(token);
        if equal(token, "struct")
            || equal(token, "union")
            || equal(token, "enum")
            || !ty2.ptr.is_none()
        {
            if counter > 0 {
                break;
            }

            if equal(token, "struct") {
                ty = struct_decl(token.nxt()).1;
                token = struct_decl(token.nxt()).0;
            } else if equal(token, "union") {
                ty = union_decl(token.nxt()).1;
                token = union_decl(token.nxt()).0;
            } else if equal(token, "enum") {
                ty = enum_specifier(token.nxt()).0;
                token = enum_specifier(token.nxt()).1;
            } else {
                ty = ty2;
                token = token.nxt();
            }
            counter += TT::OTHER as u32;
            continue;
        }

        if equal(token, "void") {
            counter += TT::VOID as u32;
        } else if equal(token, "_Bool") {
            counter += TT::BOOL as u32;
        } else if equal(token, "char") {
            counter += TT::CHAR as u32;
        } else if equal(token, "short") {
            counter += TT::SHORT as u32;
        } else if equal(token, "int") {
            counter += TT::INT as u32;
        } else if equal(token, "long") {
            counter += TT::LONG as u32;
        }

        if counter == 1 {
            ty = TyWrap::new_with_kind(TypeKind::VOID);
        } else if counter == 4 {
            ty = TyWrap::new_with_kind(TypeKind::BOOL);
        } else if counter == 16 {
            ty = TyWrap::new_with_kind(TypeKind::CHAR);
        } else if counter == 64 || counter == 320 {
            ty = TyWrap::new_with_kind(TypeKind::SHORT);
        } else if counter == 256 {
            ty = TyWrap::new_with_kind(TypeKind::INT);
        } else if counter == 1024 || counter == 1280 || counter == 2048 || counter == 2304 {
            ty = TyWrap::new_with_kind(TypeKind::LONG);
        } else {
            error_token(token, "invalid type")
        }
        token = token.nxt();
    }
    return (token, ty);
}

#[allow(dead_code)]
pub fn func_params(mut token: TokenWrap, mut ty: TyWrap) -> (TokenWrap, TyWrap) {
    let head = TyWrap::new();
    let mut cur = head;

    while !equal(token, ")") {
        if cur != head {
            token = skip(token, ",");
        }
        let (tk, ty2) = declspec(token, &mut VarAttr::empty());
        let (tk, mut ty2) = declarator(tk, ty2);

        if ty2.kind() == Some(TypeKind::ARRAY) {
            let name = ty2.name();
            ty2 = TyWrap::point_to(ty2.base());
            ty2.set_token(name);
        }

        cur.set_next(ty2);
        cur = cur.next();
        token = tk;
    }

    ty = TyWrap::new_func_ty(ty);
    ty.set_params(head.next());

    return (token.nxt(), ty);
}

#[allow(dead_code)]
pub fn array_dimensions(token: TokenWrap, ty: TyWrap) -> (TokenWrap, TyWrap) {
    if equal(token, "]") {
        let (tk, ty) = ty_suffix(token.nxt(), ty);
        return (tk, TyWrap::new_array_ty(ty, -1));
    }
    let (sz, mut token) = const_expr(token);
    token = skip(token, "]");
    let (token, ty) = ty_suffix(token, ty);
    let ty = TyWrap::new_array_ty(ty, sz as i64);
    return (token, ty);
}

#[allow(dead_code)]
pub fn ty_suffix(token: TokenWrap, ty: TyWrap) -> (TokenWrap, TyWrap) {
    if equal(token, "(") {
        return func_params(token.nxt(), ty);
    }

    if equal(token, "[") {
        return array_dimensions(token.nxt(), ty);
    }

    return (token, ty);
}

#[allow(dead_code)]
pub fn declarator(mut token: TokenWrap, mut ty: TyWrap) -> (TokenWrap, TyWrap) {
    while consume(&mut token, "*") {
        ty = TyWrap::point_to(ty);
    }

    if equal(token, "(") {
        let start = token;
        let (mut token, _) = declarator(start.nxt(), TyWrap::new());
        token = skip(token, ")");
        let (tk, ty) = ty_suffix(token, ty);
        let (_, ty) = declarator(start.nxt(), ty);

        return (tk, ty);
    }

    if token.kind() != TokenKind::IDENT {
        error_token(token, "expected a variable name");
    }

    let (tk, ty) = ty_suffix(token.nxt(), ty);
    ty.set_token(token);

    return (tk, ty);
}

#[allow(dead_code)]
pub fn abstract_declarator(mut token: TokenWrap, mut ty: TyWrap) -> (TokenWrap, TyWrap) {
    while equal(token, "*") {
        ty = TyWrap::point_to(ty);
        token = token.nxt();
    }

    if equal(token, "(") {
        let start = token;

        token = abstract_declarator(start.nxt(), TyWrap::empty()).0;

        token = skip(token, ")");
        ty = ty_suffix(token, ty).1;
        token = ty_suffix(token, ty).0;
        ty = abstract_declarator(start.nxt(), ty).1;
        return (token, ty);
    }

    return ty_suffix(token, ty);
}

#[allow(dead_code)]
pub fn type_name(token: TokenWrap) -> (TokenWrap, TyWrap) {
    let (token, ty) = declspec(token, &mut VarAttr::empty());
    return abstract_declarator(token, ty);
}

#[allow(dead_code)]
pub fn declaration(mut token: TokenWrap, base_ty: TyWrap) -> (NodeWrap, TokenWrap) {
    let head = NodeWrap::new(Num, token);
    let mut cur = head;

    let mut i = 0;
    while !equal(token, ";") {
        if i > 0 {
            token = skip(token, ",");
        }
        i += 1;

        let ty = declarator(token, base_ty).1;
        token = declarator(token, base_ty).0;

        if ty.size() < 0 {
            error_token(token, "variable has incomplete type");
        }
        if ty.kind() == Some(TypeKind::VOID) {
            error_token(token, "variable declared void");
        }
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
    let kws = [
        "void", "char", "short", "int", "long", "struct", "union", "typedef", "_Bool", "enum",
        "static",
    ];

    for kw in kws {
        if equal(token, kw) {
            return true;
        }
    }

    return !find_typedef(token).ptr.is_none();
}

#[allow(dead_code)]
fn stmt(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "return") {
        let node = NodeWrap::new(RETURN, token);
        let (expr, tk) = expr(token.nxt());

        token = skip(tk, ";");

        add_ty(expr);
        let ty = unsafe { CURRENT_FN.ty().return_ty() };
        node.set_lhs(NodeWrap::new_cast(expr, ty));
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

    if equal(token, "switch") {
        let node = NodeWrap::new(SWITCH, token);
        token = skip(token.nxt(), "(");
        let (nd, mut token) = expr(token);
        node.set_cond(nd);
        token = skip(token, ")");

        let sw = unsafe { CURRENT_SWITCH };
        unsafe { CURRENT_SWITCH = node };

        let brk = unsafe { BRAAK_LABELS };
        node.set_brk_label(new_unique_name());
        unsafe { BRAAK_LABELS = node.brk_label() };

        let (nd, token) = stmt(token);
        node.set_then(nd);
        unsafe { CURRENT_SWITCH = sw };

        unsafe { BRAAK_LABELS = brk };
        return (node, token);
    }

    if equal(token, "case") {
        if unsafe { CURRENT_SWITCH.ptr.is_none() } {
            error_token(token, "stray case")
        }

        let node = NodeWrap::new(CASE, token);
        let (val, mut token) = const_expr(token.nxt());
        token = skip(token, ":");
        node.set_label(new_unique_name());

        let (nd, token) = stmt(token);
        node.set_lhs(nd);
        node.set_val(val);

        node.set_case_next(unsafe { CURRENT_SWITCH.case_next() });
        unsafe { CURRENT_SWITCH.set_case_next(node) };

        return (node, token);
    }

    if equal(token, "default") {
        if unsafe { CURRENT_SWITCH.ptr.is_none() } {
            error_token(token, "stray default")
        }
        let node = NodeWrap::new(CASE, token);
        token = skip(token.nxt(), ":");
        node.set_label(new_unique_name());

        let (nd, token) = stmt(token);
        node.set_lhs(nd);

        unsafe { CURRENT_SWITCH.set_default_case(node) };
        return (node, token);
    }

    if equal(token, "for") {
        let node = NodeWrap::new(FOR, token);

        token = skip(token.nxt(), "(");

        let sc = ScopeWrap::new();
        sc.enter();

        let brk = unsafe { BRAAK_LABELS };
        node.set_brk_label(new_unique_name());
        unsafe { BRAAK_LABELS = node.brk_label() };

        let cont = unsafe { CONT_LABELS };
        node.set_cont_label(new_unique_name());
        unsafe { CONT_LABELS = node.cont_label() };

        if is_type_name(token) {
            let (tk, base_ty) = declspec(token, &mut VarAttr::empty());
            let (nd, tk) = declaration(tk, base_ty);
            token = tk;
            node.set_init(nd);
        } else {
            let (nd, tk) = expr_stmt(token);
            token = tk;
            node.set_init(nd);
        }

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
        sc.leave();
        unsafe { BRAAK_LABELS = brk };
        unsafe { CONT_LABELS = cont };
        return (node, token);
    }

    if equal(token, "while") {
        let node = NodeWrap::new(FOR, token);
        token = skip(token.nxt(), "(");

        let (nd, mut token) = expr(token);
        node.set_cond(nd);
        token = skip(token, ")");

        let brk = unsafe { BRAAK_LABELS };
        node.set_brk_label(new_unique_name());
        unsafe { BRAAK_LABELS = node.brk_label() };

        let cont = unsafe { CONT_LABELS };
        node.set_cont_label(new_unique_name());
        unsafe { CONT_LABELS = node.cont_label() };

        let (nd, token) = stmt(token);
        node.set_then(nd);

        unsafe { BRAAK_LABELS = brk };
        unsafe { CONT_LABELS = cont };

        return (node, token);
    }

    if equal(token, "goto") {
        let node = NodeWrap::new(GOTO, token);
        node.set_label(get_ident(token.nxt()));

        node.set_goto_next(unsafe { GOTOS });
        unsafe { GOTOS = node };
        token = skip(token.nxt().nxt(), ";");
        return (node, token);
    }

    if equal(token, "break") {
        if unsafe { BRAAK_LABELS } == "" {
            error_token(token, "stray break");
        }
        let node = NodeWrap::new(GOTO, token);
        node.set_unique_label(unsafe { BRAAK_LABELS });
        token = skip(token.nxt(), ";");

        return (node, token);
    }

    if equal(token, "continue") {
        if unsafe { CONT_LABELS } == "" {
            error_token(token, "stray continue");
        }
        let node = NodeWrap::new(GOTO, token);
        node.set_unique_label(unsafe { CONT_LABELS });
        token = skip(token.nxt(), ";");

        return (node, token);
    }

    if token.kind() == TokenKind::IDENT && equal(token.nxt(), ":") {
        let node = NodeWrap::new(NodeKind::LABEL, token);
        let label: String = token.loc().unwrap()[..token.len()].iter().collect();
        let label = Box::leak(Box::new(label));
        node.set_label(label);
        node.set_unique_label(new_unique_name());
        let (nd, tk) = stmt(token.nxt().nxt());
        node.set_lhs(nd);
        node.set_goto_next(unsafe { LABELS });
        unsafe { LABELS = node };
        return (node, tk);
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
        if is_type_name(token) && !equal(token.nxt(), ":") {
            let mut attr = VarAttr::empty();
            let base_ty = declspec(token, &mut attr).1;
            token = declspec(token, &mut attr).0;
            if attr.is_typedef.unwrap() {
                token = parse_typedef(token, base_ty);
                continue;
            }

            let (nd, tk) = declaration(token, base_ty);
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
    let (mut node, mut token) = conditional(token);
    if equal(token, "=") {
        let (n, t) = assign(token.nxt());
        node = NodeWrap::new_binary(ASSIGN, node, n, token);
        token = t;
    }

    if equal(token, "+=") {
        let (nd, tk) = assign(token.nxt());
        let (nd, _) = new_add(node, nd, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    if equal(token, "-=") {
        let (nd, tk) = assign(token.nxt());
        let (nd, _) = new_sub(node, nd, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    if equal(token, "*=") {
        let (nd, tk) = assign(token.nxt());
        let nd = NodeWrap::new_binary(NodeKind::Mul, node, nd, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    if equal(token, "/=") {
        let (nd, tk) = assign(token.nxt());
        let nd = NodeWrap::new_binary(NodeKind::Div, node, nd, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    if equal(token, "%=") {
        let (nd, tk) = assign(token.nxt());
        let nd = NodeWrap::new_binary(NodeKind::MOD, node, nd, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    if equal(token, "&=") {
        let (nd, tk) = assign(token.nxt());
        let nd = NodeWrap::new_binary(NodeKind::BITAND, node, nd, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    if equal(token, "|=") {
        let (nd, tk) = assign(token.nxt());
        let nd = NodeWrap::new_binary(NodeKind::BITOR, node, nd, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    if equal(token, "^=") {
        let (nd, tk) = assign(token.nxt());
        let nd = NodeWrap::new_binary(NodeKind::BITXOR, node, nd, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    if equal(token, ">>=") {
        let (nd, tk) = assign(token.nxt());
        let nd = NodeWrap::new_binary(NodeKind::SHR, node, nd, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    if equal(token, "<<=") {
        let (nd, tk) = assign(token.nxt());
        let nd = NodeWrap::new_binary(NodeKind::SHL, node, nd, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    return (node, token);
}

#[allow(dead_code)]
pub fn conditional(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (cond, token) = log_or(token);

    if !equal(token, "?") {
        return (cond, token);
    }

    let node = NodeWrap::new(NodeKind::COND, token);
    node.set_cond(cond);
    let (nd, mut token) = expr(token.nxt());
    node.set_then(nd);
    token = skip(token, ":");
    let (nd, token) = conditional(token);
    node.set_els(nd);
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
    let (mut node, mut token) = shift(token);

    loop {
        if equal(token, "<") {
            let (nd, tk) = shift(token.nxt());
            node = NodeWrap::new_binary(LT, node, nd, tk);
            token = tk;
            continue;
        }

        if equal(token, "<=") {
            let (nd, tk) = shift(token.nxt());
            node = NodeWrap::new_binary(LE, node, nd, tk);
            token = tk;
            continue;
        }

        if equal(token, ">") {
            let (nd, tk) = shift(token.nxt());
            node = NodeWrap::new_binary(LT, nd, node, tk);
            token = tk;
            continue;
        }

        if equal(token, ">=") {
            let (nd, tk) = shift(token.nxt());
            node = NodeWrap::new_binary(LE, nd, node, tk);
            token = tk;
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
pub fn shift(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = add(token);

    loop {
        let start = token;

        if equal(token, "<<") {
            let (nd, tk) = add(token.nxt());
            token = tk;
            node = NodeWrap::new_binary(NodeKind::SHL, node, nd, start);
            continue;
        }

        if equal(token, ">>") {
            let (nd, tk) = add(token.nxt());
            token = tk;
            node = NodeWrap::new_binary(NodeKind::SHR, node, nd, start);
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
    let num_node = NodeWrap::new_long(val, token);
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
        let num_node = NodeWrap::new_long(val, token);
        let rhs_node = NodeWrap::new_binary(Mul, rhs, num_node, token);
        add_ty(rhs_node);
        let node = NodeWrap::new_binary(Sub, lhs, rhs_node, token);
        node.set_ty(lhs.ty());
        return (node, token);
    }
    if !lhs.ty().base().ptr.is_none() && !rhs.ty().base().ptr.is_none() {
        let node = NodeWrap::new_binary(Sub, lhs, rhs, token);
        let ty = TyWrap::new_with_kind(TypeKind::INT);
        node.set_ty(ty);
        let val = lhs.ty().base().size();
        let num_node = NodeWrap::new_num(val, token);
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
            let (nd, _) = new_add(node, nd, token);
            node = nd;
            token = tk;
            continue;
        }
        if equal(token, "-") {
            let (nd, tk) = mul(token.nxt());
            let (nd, _) = new_sub(node, nd, token);
            node = nd;
            token = tk;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
fn mul(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = cast(token);

    loop {
        if equal(token, "*") {
            let (nd, tk) = cast(token.nxt());
            node = NodeWrap::new_binary(Mul, node, nd, token);
            token = tk;
            continue;
        }
        if equal(token, "/") {
            let (nd, tk) = cast(token.nxt());
            node = NodeWrap::new_binary(Div, node, nd, token);
            token = tk;
            continue;
        }
        if equal(token, "%") {
            let (nd, tk) = cast(token.nxt());
            node = NodeWrap::new_binary(MOD, node, nd, token);
            token = tk;
            continue;
        }
        return (node, token);
    }
}

#[allow(dead_code)]
pub fn cast(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "(") && is_type_name(token.nxt()) {
        let start = token;
        let ty = type_name(token.nxt()).1;
        token = type_name(token.nxt()).0;
        token = skip(token, ")");
        let expr = cast(token).0;
        token = cast(token).1;
        let node = NodeWrap::new_cast(expr, ty);
        node.set_token(start);
        return (node, token);
    }
    return unary(token);
}

#[allow(dead_code)]
fn unary(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    if equal(token, "+") {
        return cast(token.nxt());
    }
    if equal(token, "-") {
        let (nd, tk) = cast(token.nxt());
        return (NodeWrap::new_unary(NEG, nd, tk), tk);
    }
    if equal(token, "&") {
        let (nd, tk) = cast(token.nxt());
        return (NodeWrap::new_unary(ADDR, nd, tk), tk);
    }
    if equal(token, "*") {
        let (nd, tk) = cast(token.nxt());
        return (NodeWrap::new_unary(DEREF, nd, tk), tk);
    }
    if equal(token, "!") {
        let (nd, tk) = cast(token.nxt());
        return (NodeWrap::new_unary(NOT, nd, tk), tk);
    }
    if equal(token, "~") {
        let (nd, tk) = cast(token.nxt());
        return (NodeWrap::new_unary(BITNOT, nd, tk), tk);
    }
    if equal(token, "++") {
        let (nd, tk) = unary(token.nxt());
        let rhs = NodeWrap::new_num(1, token);
        let (nd, _) = new_add(nd, rhs, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    if equal(token, "--") {
        let (nd, tk) = unary(token.nxt());
        let rhs = NodeWrap::new_num(1, token);
        let (nd, _) = new_sub(nd, rhs, token);
        let nd = to_assign(nd);
        return (nd, tk);
    }

    postfix(token)
}

#[allow(dead_code)]
pub fn struct_members(mut token: TokenWrap, ty: TyWrap) -> TokenWrap {
    let head = MemberWrap::new();
    let mut cur = head;

    while !equal(token, "}") {
        let (mut tk, base_ty) = declspec(token, &mut VarAttr::empty());
        let mut first = true;

        while !consume(&mut tk, ";") {
            if !first {
                tk = skip(tk, ",");
            }
            first = false;

            let mem = MemberWrap::new();

            let (t, ty) = declarator(tk, base_ty);
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
pub fn struct_union_decl(mut token: TokenWrap) -> (TokenWrap, TyWrap) {
    let mut tag = TokenWrap::empty();
    if token.kind() == TokenKind::IDENT {
        tag = token;
        token = token.nxt();
    }

    if !tag.ptr.is_none() && !equal(token, "{") {
        let ty = find_tag(tag);
        if !ty.ptr.is_none() {
            return (token, ty);
        }

        let ty = TyWrap::new_with_kind(TypeKind::STRUCT);
        ty.set_size(-1);
        TagScopeWrap::push(tag, ty);
        return (token, ty);
    }
    token = skip(token, "{");

    let ty = TyWrap::new_with_kind(TypeKind::STRUCT);
    token = struct_members(token, ty);
    ty.set_align(1);

    if !tag.ptr.is_none() {
        for s in unsafe { SCOPE.tags() } {
            if equal(tag, s.name()) {
                s.set_ty(ty);
                return (token, s.ty());
            }
        }
        TagScopeWrap::push(tag, ty);
    }
    return (token, ty);
}

#[allow(dead_code)]
pub fn struct_decl(token: TokenWrap) -> (TokenWrap, TyWrap) {
    let (tk, ty) = struct_union_decl(token);
    ty.set_kind(TypeKind::STRUCT);

    if ty.size() < 0 {
        return (tk, ty);
    }

    let mut offset = 0;
    for mem in ty.mems() {
        offset = align_to(offset, mem.ty().align());
        mem.set_offset(offset as i64);
        offset += mem.ty().size() as usize;
        if ty.align() < mem.ty().align() {
            ty.set_align(mem.ty().align());
        }
    }
    ty.set_size(align_to(offset, ty.align()) as i64);

    return (tk, ty);
}

#[allow(dead_code)]
pub fn union_decl(token: TokenWrap) -> (TokenWrap, TyWrap) {
    let (token, ty) = struct_union_decl(token);
    ty.set_kind(TypeKind::UNION);

    for mem in ty.mems() {
        if ty.align() < mem.ty().align() {
            ty.set_align(mem.ty().align());
        }
        if ty.size() < mem.ty().size() {
            ty.set_size(mem.ty().size());
        }
    }
    ty.set_size(align_to(ty.size() as usize, ty.align()) as i64);
    return (token, ty);
}

#[allow(dead_code)]
pub fn get_struct_member(ty: TyWrap, token: TokenWrap) -> MemberWrap {
    for mem in ty.mems() {
        let len = token.len();
        if mem.name().len() == token.len()
            && (&mem.name().loc().unwrap()[..len] == &token.loc().unwrap()[..len])
        {
            return mem;
        }
    }
    error_token(token, "no such member");
    return MemberWrap::empty();
}

#[allow(dead_code)]
pub fn struct_ref(lhs: NodeWrap, token: TokenWrap) -> NodeWrap {
    add_ty(lhs);

    if lhs.ty().kind() != Some(TypeKind::STRUCT) && lhs.ty().kind() != Some(TypeKind::UNION) {
        error_token(lhs.token(), "not a struct nor a union");
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

        if equal(token, "->") {
            let nd = NodeWrap::new_unary(NodeKind::DEREF, node, token);
            let nd = struct_ref(nd, token.nxt());
            node = nd;
            token = token.nxt().nxt();
            continue;
        }

        if equal(token, "++") {
            let nd = new_inc_dec(node, token, 1);
            node = nd;
            token = token.nxt();
            continue;
        }

        if equal(token, "--") {
            let nd = new_inc_dec(node, token, -1);
            node = nd;
            token = token.nxt();
            continue;
        }

        return (node, token);
    }
}

#[allow(dead_code)]
pub fn func_call(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let start = token;
    token = token.nxt().nxt();

    let vs = find_var(start);
    if vs.ptr.is_none() {
        error_token(start, "implicit declaration of a function")
    }
    if vs.var().ptr.is_none() || vs.var().ty().kind() != Some(TypeKind::FUNC) {
        error_token(start, "not a function")
    }

    let ty = vs.var().ty();
    let mut param_ty = ty.params();

    let head = NodeWrap::new(Num, token);
    let mut cur = head;

    while !equal(token, ")") {
        if cur != head {
            token = skip(token, ",");
        }

        let (mut arg, t) = assign(token);
        add_ty(arg);

        if !param_ty.ptr.is_none() {
            if param_ty.kind() == Some(TypeKind::STRUCT) || param_ty.kind() == Some(TypeKind::UNION)
            {
                error_token(arg.token(), "passing struct or union is not supported yet");
            }
            arg = NodeWrap::new_cast(arg, param_ty);
            param_ty = param_ty.next();
        }

        cur.set_nxt(arg);
        cur = cur.nxt();
        token = t;
        add_ty(cur);
    }
    token = skip(token, ")");

    let node = NodeWrap::new(FUNCALL, start);
    let len = start.len();
    let func_name = start.loc().unwrap()[..len].iter().collect::<String>();
    node.set_func_name(Box::leak(Box::new(func_name)));
    node.set_func_type(ty);
    node.set_ty(ty.return_ty());

    node.set_args(head.nxt());

    return (node, token);
}

#[allow(dead_code)]
fn primary(mut token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let start = token;
    if equal(token, "(") && equal(token.nxt(), "{") {
        let node = NodeWrap::new(NodeKind::STMTEXPR, token);
        let (nd, tk) = compound_stmt(token.nxt().nxt());
        node.set_body(nd.body());
        token = skip(tk, ")");
        return (node, token);
    }

    if equal(token, "(") {
        let (nd, mut token) = expr(token.nxt());
        token = skip(token, ")");
        return (nd, token);
    }

    if equal(token, "sizeof") && equal(token.nxt(), "(") && is_type_name(token.nxt().nxt()) {
        let (mut token, ty) = type_name(token.nxt().nxt());
        token = skip(token, ")");
        let node = NodeWrap::new_num(ty.size() as i64, start);
        return (node, token);
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

        let vs = find_var(token);
        if vs.ptr.is_none() || (vs.var().ptr.is_none() && vs.enum_ty().ptr.is_none()) {
            error_token(token, "undefined variable");
        }

        let node: NodeWrap;
        if !vs.var().ptr.is_none() {
            node = NodeWrap::new_var_node(vs.var(), token);
        } else {
            node = NodeWrap::new_num(vs.enum_val(), token)
        }

        return (node, token.nxt());
    }

    if token.kind() == TokenKind::STR {
        let var = new_string_literal(token.stri(), token.ty());
        token = token.nxt();
        return (NodeWrap::new_var_node(var, token), token);
    }

    if token.kind() == TokenKind::NUM {
        let node = NodeWrap::new_num(token.val() as i64, token);
        return (node, token.nxt());
    }

    error_token(token, "expected an expression");
    (NodeWrap::empty(), token)
}

#[allow(dead_code)]
pub fn parse_typedef(mut token: TokenWrap, base_ty: TyWrap) -> TokenWrap {
    let mut first = true;

    while !consume(&mut token, ";") {
        if !first {
            token = skip(token, ",")
        }
        first = false;
        let ty = declarator(token, base_ty).1;
        token = declarator(token, base_ty).0;
        ScopeWrap::push(get_ident(ty.name())).set_typedef(ty);
    }

    return token;
}

#[allow(dead_code)]
pub fn create_param_l_vars(params: TyWrap) {
    if !params.ptr.is_none() {
        create_param_l_vars(params.next());
        ObjWrap::new_local(get_ident(params.name()), params);
    }
}

#[allow(dead_code)]
pub fn resolve_goto_labels() {
    let mut g = unsafe { GOTOS };

    while !g.ptr.is_none() {
        let mut l = unsafe { LABELS };
        while !l.ptr.is_none() {
            if g.label() == l.label() {
                g.set_unique_label(l.unique_label());
                break;
            }
            l = l.goto_next();
        }
        if g.unique_label() == "" {
            error_token(g.token().nxt(), "use of undeclared label")
        }
        g = g.goto_next();
    }

    unsafe { GOTOS = NodeWrap::empty() };
    unsafe { LABELS = NodeWrap::empty() };
}

#[allow(dead_code)]
pub fn function(token: TokenWrap, base_ty: TyWrap, attr: VarAttr) -> (ObjWrap, TokenWrap) {
    let (mut token, typ) = declarator(token, base_ty);

    let func = ObjWrap::new_global(get_ident(typ.name()), typ);
    func.set_is_function(true);
    func.set_is_definition(!consume(&mut token, ";"));
    func.set_is_static(attr.is_static.unwrap());

    if !func.is_definition() {
        return (func, token);
    }

    unsafe { CURRENT_FN = func }

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
    resolve_goto_labels();

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
        let (tk, ty) = declarator(token, base_ty);
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
    let ty = declarator(token, dummy).1;
    return ty.kind() == Some(TypeKind::FUNC);
}

#[allow(dead_code)]
pub fn parse(mut token: TokenWrap) -> ObjWrap {
    unsafe { SCOPE = ScopeWrap::new() }
    unsafe { GLOBALS = ObjWrap::empty() };

    while token.kind() != TokenKind::EOF {
        let mut attr = VarAttr::empty();
        let (tk, base_ty) = declspec(token, &mut attr);
        token = tk;

        if attr.is_typedef.unwrap() {
            token = parse_typedef(token, base_ty);
            continue;
        }
        if is_function(tk) {
            token = function(tk, base_ty, attr).1;
            continue;
        }
        token = global_variable(tk, base_ty);
    }

    return unsafe { GLOBALS };
}

#[allow(dead_code)]
pub fn enum_specifier(mut token: TokenWrap) -> (TyWrap, TokenWrap) {
    let ty = TyWrap::new_with_kind(TypeKind::ENUM);

    let mut tag = TokenWrap::empty();
    if token.kind() == TokenKind::IDENT {
        tag = token;
        token = token.nxt();
    }

    if !tag.ptr.is_none() && !equal(token, "{") {
        let ty = find_tag(tag);
        if ty.ptr.is_none() {
            error_token(tag, "unknown enum type");
        }
        if ty.kind() != Some(TypeKind::ENUM) {
            error_token(tag, "not an enum tag");
        }
        return (ty, token);
    }
    token = skip(token, "{");

    let mut i = 0;
    let mut val = 0;

    while !equal(token, "}") {
        if i > 0 {
            token = skip(token, ",");
        }
        i += 1;
        let name = get_ident(token);
        token = token.nxt();

        if equal(token, "=") {
            val = const_expr(token.nxt()).0;
            token = const_expr(token.nxt()).1;
        }
        let vs = ScopeWrap::push(name);
        vs.set_enum_ty(ty);
        vs.set_enum_val(val);
        val += 1;
    }
    if !tag.ptr.is_none() {
        TagScopeWrap::push(tag, ty);
    }

    return (ty, token.nxt());
}

#[allow(dead_code)]
pub fn to_assign(binary: NodeWrap) -> NodeWrap {
    add_ty(binary.lhs());
    add_ty(binary.rhs());

    let token = binary.token();
    let var = ObjWrap::new_local("", TyWrap::point_to(binary.lhs().ty()));

    let expr1 = NodeWrap::new_binary(
        NodeKind::ASSIGN,
        NodeWrap::new_var_node(var, token),
        NodeWrap::new_unary(NodeKind::ADDR, binary.lhs(), token),
        token,
    );

    let expr2 = NodeWrap::new_binary(
        NodeKind::ASSIGN,
        NodeWrap::new_unary(NodeKind::DEREF, NodeWrap::new_var_node(var, token), token),
        NodeWrap::new_binary(
            binary.kind(),
            NodeWrap::new_unary(NodeKind::DEREF, NodeWrap::new_var_node(var, token), token),
            binary.rhs(),
            token,
        ),
        token,
    );

    return NodeWrap::new_binary(NodeKind::COMMA, expr1, expr2, token);
}

#[allow(dead_code)]
pub fn new_inc_dec(node: NodeWrap, token: TokenWrap, add_end: i64) -> NodeWrap {
    add_ty(node);
    let (nd, _) = new_add(node, NodeWrap::new_num(add_end, token), token);
    let (nd, _) = new_add(to_assign(nd), NodeWrap::new_num(-add_end, token), token);
    return NodeWrap::new_cast(nd, node.ty());
}

#[allow(dead_code)]
pub fn bit_and(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = equality(token);
    while equal(token, "&") {
        let start = token;
        let (nd, tk2) = equality(token.nxt());
        token = tk2;
        let nd = NodeWrap::new_binary(NodeKind::BITAND, node, nd, start);
        node = nd;
    }
    return (node, token);
}

#[allow(dead_code)]
pub fn bit_xor(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = bit_and(token);
    while equal(token, "^") {
        let start = token;
        let (nd, tk2) = bit_and(token.nxt());
        token = tk2;
        let nd = NodeWrap::new_binary(NodeKind::BITXOR, node, nd, start);
        node = nd;
    }
    return (node, token);
}

#[allow(dead_code)]
pub fn bit_or(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = bit_xor(token);
    while equal(token, "|") {
        let start = token;
        let (nd, tk2) = bit_xor(token.nxt());
        token = tk2;
        let nd = NodeWrap::new_binary(NodeKind::BITOR, node, nd, start);
        node = nd;
    }
    return (node, token);
}

#[allow(dead_code)]
pub fn log_and(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = bit_or(token);
    while equal(token, "&&") {
        let start = token;
        let (nd, tk2) = bit_or(token.nxt());
        token = tk2;
        let nd = NodeWrap::new_binary(NodeKind::LOGAND, node, nd, start);
        node = nd;
    }
    return (node, token);
}

#[allow(dead_code)]
pub fn log_or(token: TokenWrap) -> (NodeWrap, TokenWrap) {
    let (mut node, mut token) = log_and(token);
    while equal(token, "||") {
        let start = token;
        let (nd, tk2) = log_and(token.nxt());
        token = tk2;
        let nd = NodeWrap::new_binary(NodeKind::LOGOR, node, nd, start);
        node = nd;
    }
    return (node, token);
}

#[allow(dead_code)]
pub fn eval(node: NodeWrap) -> i64 {
    add_ty(node);

    if node.ptr.is_none() {
        return i64::MAX
    }

    match node.kind() {
        NodeKind::Add => return eval(node.lhs()) + eval(node.rhs()),
        NodeKind::Sub => return eval(node.lhs()) - eval(node.rhs()),
        NodeKind::Mul => return eval(node.lhs()) * eval(node.rhs()),
        NodeKind::Div => return eval(node.lhs()) / eval(node.rhs()),
        NodeKind::NEG => return -eval(node.lhs()),
        NodeKind::MOD => return eval(node.lhs()) % eval(node.rhs()),
        NodeKind::BITAND => return eval(node.lhs()) & eval(node.rhs()),
        NodeKind::BITOR => return eval(node.lhs()) | eval(node.rhs()),
        NodeKind::BITXOR => return eval(node.lhs()) ^ eval(node.rhs()),
        NodeKind::SHL => return eval(node.lhs()) << eval(node.rhs()),
        NodeKind::SHR => return eval(node.lhs()) >> eval(node.rhs()),
        NodeKind::EQ => return (eval(node.lhs()) == eval(node.rhs())) as i64,
        NodeKind::NE => return (eval(node.lhs()) != eval(node.rhs())) as i64,
        NodeKind::LT => return (eval(node.lhs()) < eval(node.rhs())) as i64,
        NodeKind::LE => return (eval(node.lhs()) <= eval(node.rhs())) as i64,
        NodeKind::COND => {
            if eval(node.cond()) > 0 {
                return eval(node.then());
            } else {
                return eval(node.els());
            }
        }
        NodeKind::COMMA => return eval(node.rhs()),
        NodeKind::NOT => return !(eval(node.lhs()).is_positive()) as i64,
        NodeKind::BITNOT => return !eval(node.lhs()),
        NodeKind::LOGAND => {
            return ((eval(node.lhs()).is_positive()) && eval(node.rhs()).is_positive()) as i64
        }
        NodeKind::LOGOR => {
            return ((eval(node.lhs()).is_positive()) || eval(node.rhs()).is_positive()) as i64
        }
        NodeKind::CAST => {
            if is_int(node.ty()) {
                match node.ty().size() {
                    1 => return eval(node.lhs()),
                    2 => return eval(node.lhs()),
                    4 => return eval(node.lhs()),
                    _ => {}
                }
            }
            return eval(node.lhs());
        }
        NodeKind::Num => return node.val(),

        _ => return i64::MAX,
    }
}

#[allow(dead_code)]
pub fn const_expr(token: TokenWrap) -> (i64, TokenWrap){
    let (node, token) = conditional(token);
    return (eval(node), token);
}
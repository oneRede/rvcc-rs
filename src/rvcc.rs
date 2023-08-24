use crate::parse::LOCALS;

#[allow(dead_code)]
pub static mut TYPE_INT: TyWrap = TyWrap::empty();

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    IDENT,
    Punct,
    Num,
    EOF,
    KEYWORD,
}

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Punct => "Punct".to_string(),
            TokenKind::IDENT => "IDENT".to_string(),
            TokenKind::Num => "Num".to_string(),
            TokenKind::EOF => "EOF".to_string(),
            TokenKind::KEYWORD => "KEYWORD".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub next: TokenWrap,
    pub val: i32,
    pub loc: Option<&'static [char]>,
    pub len: usize,
}

#[allow(dead_code)]
impl Iterator for TokenWrap {
    type Item = TokenWrap;

    fn next(&mut self) -> Option<Self::Item> {
        let now = *self;
        if !now.ptr.is_none() {
            self.ptr = self.nxt().ptr;
            return Some(now);
        } else {
            return None;
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct TokenWrap {
    pub ptr: Option<*mut Token>,
}

#[allow(dead_code)]
impl TokenWrap {
    pub fn new(token_kind: TokenKind, loc: &'static [char], len: usize) -> Self {
        let tk = Token{
            kind: token_kind,
            next: TokenWrap::empty(),
            val: 0,
            loc: Some(loc),
            len: len,
        };
        let tk: Option<*mut Token> = Some(Box::leak(Box::new(tk)));
        Self { ptr: tk }
    }

    pub fn init() -> Self {
        let tk = Token{
            kind: TokenKind::Num,
            next: TokenWrap::empty(),
            val: 0,
            loc:None,
            len: 0,
        };
        let tk: Option<*mut Token> = Some(Box::leak(Box::new(tk)));
        Self { ptr: tk }
    }

    pub fn empty() -> Self {
        Self {
            ptr: None,
        }
    }

    pub fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }

    pub fn reset_by_next(&mut self) -> Self {
        *self = unsafe { self.ptr.unwrap().as_ref().unwrap().next };
        *self
    }

    pub fn set(&mut self, ptr: Option<*mut Token>) -> Self {
        self.ptr = ptr;
        *self
    }

    pub fn set_next(self, next: TokenWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next };
    }

    pub fn set_kind(self, kind: TokenKind) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().kind = kind };
    }

    pub fn set_val(self, val: i32) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().val = val };
    }

    pub fn set_len(self, len: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().len = len };
    }

    pub fn nxt(&self) -> TokenWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn kind(&self) -> TokenKind {
        unsafe { self.ptr.unwrap().as_ref().unwrap().kind }
    }

    pub fn get_len(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().len }
    }

    pub fn val(&self) -> i32 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().val }
    }

    pub fn get_loc(&self) -> Option<&[char]> {
        unsafe { self.ptr.unwrap().as_ref().unwrap().loc }
    }

    pub fn get_ref(&self) -> &Token {
        unsafe { self.ptr.unwrap().as_ref().unwrap() }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num,
    NEG,
    EQ,
    NE,
    LT,
    LE,
    ExprStmt,
    ASSIGN,
    VAR,
    RETURN,
    BLOCK,
    IF,
    FOR,
    ADDR,
    DEREF,
    FUNCALL,
}

impl ToString for NodeKind {
    fn to_string(&self) -> String {
        match self {
            NodeKind::Add => "Add".to_string(),
            NodeKind::Sub => "Sub".to_string(),
            NodeKind::Mul => "Mul".to_string(),
            NodeKind::Div => "Div".to_string(),
            NodeKind::Num => "Num".to_string(),
            NodeKind::NEG => "NEG".to_string(),
            NodeKind::EQ => "EQ".to_string(),
            NodeKind::NE => "NE".to_string(),
            NodeKind::LT => "LT".to_string(),
            NodeKind::LE => "LE".to_string(),
            NodeKind::ExprStmt => "ExprStmt".to_string(),
            NodeKind::ASSIGN => "ASSIGN".to_string(),
            NodeKind::VAR => "VAR".to_string(),
            NodeKind::RETURN => "RETURN".to_string(),
            NodeKind::BLOCK => "BLOCK".to_string(),
            NodeKind::IF => "IF".to_string(),
            NodeKind::FOR => "FOR".to_string(),
            NodeKind::ADDR => "ADDR".to_string(),
            NodeKind::DEREF => "DEREF".to_string(),
            NodeKind::FUNCALL => "FUNCALL".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct NodeV2 {
    pub kind: NodeKind,
    pub next: NodeWrap,
    pub lhs: NodeWrap,
    pub rhs: NodeWrap,
    pub body: NodeWrap,
    pub cond: NodeWrap,
    pub then: NodeWrap,
    pub els: NodeWrap,
    pub val: i64,
    pub var: ObjWrap,
    pub init: NodeWrap,
    pub inc: NodeWrap,
    pub token: TokenWrap,
    pub ty: TyWrap,
    pub func_name: &'static str,
    pub args: NodeWrap,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeWrap {
    pub ptr: Option<*mut NodeV2>,
}

#[allow(dead_code)]
impl Iterator for NodeWrap {
    type Item = NodeWrap;

    fn next(&mut self) -> Option<Self::Item> {
        let now = *self;
        if !now.ptr.is_none() {
            self.ptr = self.nxt().ptr;
            return Some(now);
        } else {
            return None;
        }
    }
}

#[allow(dead_code)]
impl NodeWrap {
    pub fn new_node_wrap(node: Option<*mut NodeV2>) -> Self {
        Self { ptr: node }
    }

    pub fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn kind(&self) -> NodeKind {
        unsafe { self.ptr.unwrap().as_ref().unwrap().kind }
    }

    pub fn nxt(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn lhs(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().lhs }
    }

    pub fn rhs(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().rhs }
    }

    pub fn body(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().body }
    }

    pub fn cond(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().cond }
    }

    pub fn then(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().then }
    }

    pub fn els(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().els }
    }

    pub fn val(&self) -> i64 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().val }
    }

    pub fn var(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().var }
    }

    pub fn init(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().init }
    }

    pub fn inc(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().inc }
    }

    pub fn token(&self) -> TokenWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().token }
    }

    pub fn ty(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().ty }
    }

    pub fn func_name(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().func_name }
    }

    pub fn args(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().args }
    }

    pub fn set_kind(&self, kind: NodeKind) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().kind = kind }
    }

    pub fn set_nxt(&self, next: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_lhs(&self, lhs: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().lhs = lhs }
    }

    pub fn set_rhs(&self, rhs: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().rhs = rhs }
    }

    pub fn set_body(&self, body: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().body = body }
    }

    pub fn set_cond(&self, cond: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().cond = cond }
    }

    pub fn set_then(&self, then: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().then = then }
    }

    pub fn set_els(&self, els: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().els = els }
    }

    pub fn set_val(&self, val: i64) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().val = val }
    }

    pub fn set_var(&self, var: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().var = var }
    }

    pub fn set_init(&self, init: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().init = init }
    }

    pub fn set_inc(&self, inc: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().inc = inc }
    }

    pub fn set_token(&self, token: TokenWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().token = token }
    }

    pub fn set_ty(&self, ty: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().ty = ty }
    }

    pub fn set_func_name(&self, func_name: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().func_name = func_name }
    }

    pub fn set_args(&self, args: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().args = args }
    }
}

#[allow(dead_code)]
impl NodeWrap {
    pub fn new(kind: NodeKind, token: TokenWrap) -> NodeWrap {
        let node = NodeV2 {
            kind: kind,
            next: NodeWrap::empty(),
            lhs: NodeWrap::empty(),
            rhs: NodeWrap::empty(),
            body: NodeWrap::empty(),
            cond: NodeWrap::empty(),
            then: NodeWrap::empty(),
            els: NodeWrap::empty(),
            val: 0,
            var: ObjWrap::empty(),
            init: NodeWrap::empty(),
            inc: NodeWrap::empty(),
            token: token,
            ty: TyWrap::empty(),
            func_name: "",
            args: NodeWrap::empty(),
        };
        let node: Option<*mut NodeV2> = Some(Box::leak(Box::new(node)));
        NodeWrap::new_node_wrap(node)
    }

    pub fn new_binary(kind: NodeKind, lhs: NodeWrap, rhs: NodeWrap, token: TokenWrap) -> NodeWrap {
        let node = NodeV2 {
            kind: kind,
            next: NodeWrap::empty(),
            lhs: lhs,
            rhs: rhs,
            body: NodeWrap::empty(),
            cond: NodeWrap::empty(),
            then: NodeWrap::empty(),
            els: NodeWrap::empty(),
            val: 0,
            var: ObjWrap::empty(),
            init: NodeWrap::empty(),
            inc: NodeWrap::empty(),
            token: token,
            ty: TyWrap::empty(),
            func_name: "",
            args: NodeWrap::empty(),
        };
        let node: Option<*mut NodeV2> = Some(Box::leak(Box::new(node)));
        NodeWrap::new_node_wrap(node)
    }

    pub fn new_num(val: i64, token: TokenWrap) -> NodeWrap {
        let node = NodeV2 {
            kind: NodeKind::Num,
            next: NodeWrap::empty(),
            lhs: NodeWrap::empty(),
            rhs: NodeWrap::empty(),
            body: NodeWrap::empty(),
            cond: NodeWrap::empty(),
            then: NodeWrap::empty(),
            els: NodeWrap::empty(),
            val: val,
            var: ObjWrap::empty(),
            init: NodeWrap::empty(),
            inc: NodeWrap::empty(),
            token: token,
            ty: TyWrap::new_with_kind(Some(TypeKind::INT)),
            func_name: "",
            args: NodeWrap::empty(),
        };
        let node: Option<*mut NodeV2> = Some(Box::leak(Box::new(node)));
        NodeWrap::new_node_wrap(node)
    }

    pub fn new_unary(kind: NodeKind, expr: NodeWrap, token: TokenWrap) -> NodeWrap {
        let node: NodeWrap = NodeWrap::new(kind, token);
        node.set_lhs(expr);
        return node;
    }

    pub fn new_var_node(var: ObjWrap, token: TokenWrap) -> NodeWrap {
        let node = NodeV2 {
            kind: NodeKind::VAR,
            next: NodeWrap::empty(),
            lhs: NodeWrap::empty(),
            rhs: NodeWrap::empty(),
            body: NodeWrap::empty(),
            cond: NodeWrap::empty(),
            then: NodeWrap::empty(),
            els: NodeWrap::empty(),
            val: 0,
            var: var,
            init: NodeWrap::empty(),
            inc: NodeWrap::empty(),
            token: token,
            ty: TyWrap::empty(),
            func_name: "",
            args: NodeWrap::empty(),
        };
        let node: Option<*mut NodeV2> = Some(Box::leak(Box::new(node)));
        NodeWrap::new_node_wrap(node)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Obj {
    pub next: ObjWrap,
    pub name: &'static str,
    pub offset: i64,
    pub ty: TyWrap,
}
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct ObjWrap {
    pub ptr: Option<*mut Obj>,
}

#[allow(dead_code)]
impl ObjWrap {
    pub fn new(name: &'static str, ty: TyWrap) -> Self {
        let var = Obj {
            next: ObjWrap::empty(),
            name: name,
            offset: 0,
            ty: ty,
        };
        let var: Option<*mut Obj> = Some(Box::leak(Box::new(var)));
        let var = Self { ptr: var };
        var.set_nxt(unsafe { LOCALS });
        unsafe { LOCALS = var };
        var
    }

    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn nxt(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn name(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().name }
    }

    pub fn offset(&self) -> i64 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().offset }
    }

    pub fn ty(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().ty }
    }

    pub fn set_nxt(&self, next: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_name(&self, name: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().name = name }
    }

    pub fn set_offset(&self, offset: i64) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().offset = offset }
    }

    pub fn set_ty(&self, ty: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().ty = ty }
    }
}

#[allow(dead_code)]
impl Iterator for ObjWrap {
    type Item = ObjWrap;

    fn next(&mut self) -> Option<Self::Item> {
        let now = *self;
        if !now.ptr.is_none() {
            self.ptr = self.nxt().ptr;
            return Some(now);
        } else {
            return None;
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Function {
    pub next: FunctionWrap,
    pub name: &'static str,
    pub body: NodeWrap,
    pub locals: ObjWrap,
    pub stack_size: i64,
    pub params: ObjWrap,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct FunctionWrap {
    pub ptr: Option<*mut Function>,
}

#[allow(dead_code)]
impl Iterator for FunctionWrap {
    type Item = FunctionWrap;

    fn next(&mut self) -> Option<Self::Item> {
        let now = *self;
        if !now.ptr.is_none() {
            self.ptr = self.nxt().ptr;
            return Some(now);
        } else {
            return None;
        }
    }
}

#[allow(dead_code)]
impl FunctionWrap {
    pub fn new(body: NodeWrap, locals: ObjWrap) -> Self {
        let func = Function {
            next: FunctionWrap::empty(),
            name: "",
            body: body,
            locals: locals,
            stack_size: 0,
            params: ObjWrap::empty(),
        };
        let func: Option<*mut Function> = Some(Box::leak(Box::new(func)));
        Self { ptr: func }
    }

    pub fn init() -> Self {
        let func = Function {
            next: FunctionWrap::empty(),
            name: "",
            body: NodeWrap::empty(),
            locals: ObjWrap::empty(),
            stack_size: 0,
            params: ObjWrap::empty(),
        };
        let func: Option<*mut Function> = Some(Box::leak(Box::new(func)));
        Self { ptr: func }
    }

    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn nxt(&self) -> FunctionWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn name(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().name }
    }

    pub fn body(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().body }
    }

    pub fn locals(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().locals }
    }

    pub fn stack_size(&self) -> i64 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().stack_size }
    }

    pub fn params(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().params }
    }

    pub fn set_nxt(&self, next: FunctionWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_name(&self, name: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().name = name }
    }

    pub fn set_body(&self, body: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().body = body }
    }

    pub fn set_locals(&self, locals: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().locals = locals }
    }

    pub fn set_stack_size(&self, stack_size: i64) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().stack_size = stack_size }
    }

    pub fn set_params(&self, params: ObjWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().params = params }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TypeKind {
    INT,
    PTR,
    FUNC,
    ARRAY,
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TyV2 {
    pub kind: Option<TypeKind>,
    pub base: TyWrap,
    pub token: TokenWrap,
    pub return_ty: TyWrap,
    pub params: TyWrap,
    pub next: TyWrap,
    pub size: usize,
    pub array_len: usize,
}

impl TyV2 {
    pub fn new() -> Self {
        Self {
            kind: None,
            base: TyWrap::empty(),
            token: TokenWrap::empty(),
            return_ty: TyWrap::empty(),
            params: TyWrap::empty(),
            next: TyWrap::empty(),
            size: 8,
            array_len: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TyWrap {
    pub ptr: Option<*mut TyV2>,
}

#[allow(dead_code)]
impl TyWrap {
    pub fn new() -> Self {
        let ty: Option<*mut TyV2> = Some(Box::leak(Box::new(TyV2::new())));
        Self { ptr: ty }
    }

    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn new_with_kind(kind: Option<TypeKind>) -> Self {
        let ty = TyWrap::new();
        ty.set_kind(kind);
        ty
    }

    pub fn new_func_ty(return_ty: TyWrap) -> Self {
        let ty = TyWrap::new();
        ty.set_kind(Some(TypeKind::FUNC));
        ty.set_return_ty(return_ty);
        ty
    }

    pub fn new_array_ty(base: TyWrap, len: usize) -> Self {
        let ty = TyWrap::new();
        ty.set_base(base);
        ty.set_kind(Some(TypeKind::ARRAY));
        ty.set_size(ty.base().size() * len);
        ty.set_array_len(len);
        ty
    }

    pub fn point_to(base: TyWrap) -> Self {
        let ty = TyWrap::new();
        ty.set_kind(Some(TypeKind::PTR));
        ty.set_base(base);
        ty
    }

    pub fn copy(ty: TyWrap) -> Self {
        let tmp = TyWrap::new();
        tmp.set_kind(ty.kind());
        tmp.set_base(ty.base());
        tmp.set_token(ty.token());
        tmp.set_return_ty(ty.return_ty());
        tmp.set_params(ty.params());
        tmp.set_next(ty.next());
        tmp
    }

    pub fn kind(&self) -> Option<TypeKind> {
        unsafe { self.ptr.unwrap().as_ref().unwrap().kind }
    }

    pub fn base(&self) -> Self {
        unsafe { self.ptr.unwrap().as_ref().unwrap().base }
    }

    pub fn token(&self) -> TokenWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().token }
    }

    pub fn return_ty(&self) -> Self {
        unsafe { self.ptr.unwrap().as_ref().unwrap().return_ty }
    }

    pub fn params(&self) -> Self {
        unsafe { self.ptr.unwrap().as_ref().unwrap().params }
    }

    pub fn next(&self) -> Self {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn size(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().size }
    }

    pub fn array_len(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().array_len }
    }

    pub fn set_kind(&self, kind: Option<TypeKind>) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().kind = kind };
    }

    pub fn set_base(&self, base: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().base = base };
    }

    pub fn set_token(&self, token: TokenWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().token = token };
    }

    pub fn set_return_ty(&self, return_ty: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().return_ty = return_ty }
    }

    pub fn set_params(&self, params: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().params = params }
    }

    pub fn set_next(&self, next: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_size(&self, size: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().size = size }
    }

    pub fn set_array_len(&self, array_len: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().array_len = array_len }
    }
}

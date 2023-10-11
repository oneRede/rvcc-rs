use crate::{
    obj::ObjWrap,
    token::TokenWrap,
    ty::{add_ty, TyWrap, TypeKind},
};

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
    EXPRSTMT,
    ASSIGN,
    VAR,
    RETURN,
    BLOCK,
    IF,
    FOR,
    ADDR,
    DEREF,
    FUNCALL,
    STMTEXPR,
    COMMA,
    MEMBER,
    CAST,
    NOT,
    BITNOT,
    MOD,
    BITAND,
    BITOR,
    BITXOR,
    LOGAND,
    LOGOR,
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
            NodeKind::EXPRSTMT => "ExprStmt".to_string(),
            NodeKind::ASSIGN => "ASSIGN".to_string(),
            NodeKind::VAR => "VAR".to_string(),
            NodeKind::RETURN => "RETURN".to_string(),
            NodeKind::BLOCK => "BLOCK".to_string(),
            NodeKind::IF => "IF".to_string(),
            NodeKind::FOR => "FOR".to_string(),
            NodeKind::ADDR => "ADDR".to_string(),
            NodeKind::DEREF => "DEREF".to_string(),
            NodeKind::FUNCALL => "FUNCALL".to_string(),
            NodeKind::STMTEXPR => "STMTEXPR".to_string(),
            NodeKind::COMMA => "COMMA".to_string(),
            NodeKind::MEMBER => "MEMBER".to_string(),
            NodeKind::CAST => "CAST".to_string(),
            NodeKind::NOT => "NOT".to_string(),
            NodeKind::BITNOT => "BITNOT".to_string(),
            NodeKind::MOD => "MOD".to_string(),
            NodeKind::BITAND => "BITAND".to_string(),
            NodeKind::BITOR => "BITOR".to_string(),
            NodeKind::BITXOR => "BITXOR".to_string(),
            NodeKind::LOGAND => "LOGAND".to_string(),
            NodeKind::LOGOR => "LOGOR".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Node {
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
    pub mem: MemberWrap,
    pub func_type: TyWrap,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeWrap {
    pub ptr: Option<*mut Node>,
}

#[allow(dead_code)]
impl NodeWrap {
    pub fn new(kind: NodeKind, token: TokenWrap) -> NodeWrap {
        let node = Node {
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
            mem: MemberWrap::empty(),
            func_type: TyWrap::empty(),
        };
        let node: Option<*mut Node> = Some(Box::leak(Box::new(node)));
        NodeWrap::new_node_wrap(node)
    }

    pub fn new_binary(kind: NodeKind, lhs: NodeWrap, rhs: NodeWrap, token: TokenWrap) -> NodeWrap {
        let node = Node {
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
            mem: MemberWrap::empty(),
            func_type: TyWrap::empty(),
        };
        let node: Option<*mut Node> = Some(Box::leak(Box::new(node)));
        NodeWrap::new_node_wrap(node)
    }

    pub fn new_num(val: i64, token: TokenWrap) -> NodeWrap {
        let node = Node {
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
            ty: TyWrap::new_with_kind(TypeKind::INT),
            func_name: "",
            args: NodeWrap::empty(),
            mem: MemberWrap::empty(),
            func_type: TyWrap::empty(),
        };
        let node: Option<*mut Node> = Some(Box::leak(Box::new(node)));
        NodeWrap::new_node_wrap(node)
    }

    pub fn new_long(val: i64, token: TokenWrap) -> NodeWrap {
        let node = Node {
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
            ty: TyWrap::new_with_kind(TypeKind::LONG),
            func_name: "",
            args: NodeWrap::empty(),
            mem: MemberWrap::empty(),
            func_type: TyWrap::empty(),
        };
        let node: Option<*mut Node> = Some(Box::leak(Box::new(node)));
        NodeWrap::new_node_wrap(node)
    }

    pub fn new_unary(kind: NodeKind, expr: NodeWrap, token: TokenWrap) -> NodeWrap {
        let node: NodeWrap = NodeWrap::new(kind, token);
        node.set_lhs(expr);
        return node;
    }

    pub fn new_var_node(var: ObjWrap, token: TokenWrap) -> NodeWrap {
        let node = Node {
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
            mem: MemberWrap::empty(),
            func_type: TyWrap::empty(),
        };
        let node: Option<*mut Node> = Some(Box::leak(Box::new(node)));
        NodeWrap::new_node_wrap(node)
    }

    pub fn new_cast(expr: NodeWrap, ty: TyWrap) -> NodeWrap {
        add_ty(expr);

        let node = Self::new(NodeKind::CAST, expr.token());
        node.set_lhs(expr);
        node.set_ty(ty);
        node
    }

    pub fn new_node_wrap(node: Option<*mut Node>) -> Self {
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

    pub fn mem(&self) -> MemberWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().mem }
    }

    pub fn func_type(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().func_type }
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

    pub fn set_mem(&self, mem: MemberWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().mem = mem }
    }

    pub fn set_func_type(&self, func_type: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().func_type = func_type }
    }
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
impl ToString for NodeWrap {
    fn to_string(&self) -> String {
        if self.ptr.is_none() {
            return "None".to_string();
        }

        let s_next: String;
        if self.nxt().ptr.is_none() {
            s_next = "\"None\"".to_string();
        } else {
            s_next = self.nxt().to_string()
        }

        let s_lhs: String;
        if self.lhs().ptr.is_none() {
            s_lhs = "\"None\"".to_string();
        } else {
            s_lhs = self.lhs().to_string()
        }

        let s_rhs: String;
        if self.rhs().ptr.is_none() {
            s_rhs = "\"None\"".to_string();
        } else {
            s_rhs = self.rhs().to_string()
        }

        let s_body: String;
        if self.body().ptr.is_none() {
            s_body = "\"None\"".to_string();
        } else {
            s_body = self.body().to_string()
        }

        let s_cond: String;
        if self.cond().ptr.is_none() {
            s_cond = "\"None\"".to_string();
        } else {
            s_cond = self.cond().to_string()
        }

        let s_then: String;
        if self.then().ptr.is_none() {
            s_then = "\"None\"".to_string();
        } else {
            s_then = self.then().to_string()
        }

        let s_els: String;
        if self.els().ptr.is_none() {
            s_els = "\"None\"".to_string();
        } else {
            s_els = self.els().to_string()
        }

        let s_init: String;
        if self.init().ptr.is_none() {
            s_init = "\"None\"".to_string();
        } else {
            s_init = self.init().to_string()
        }

        let s_inc: String;
        if self.inc().ptr.is_none() {
            s_inc = "\"None\"".to_string();
        } else {
            s_inc = self.inc().to_string()
        }

        let s_args: String;
        if self.args().ptr.is_none() {
            s_args = "\"None\"".to_string();
        } else {
            s_args = self.args().to_string()
        }

        let s = "{".to_string()
            + "\"kind\":"
            + "\""
            + &self.kind().to_string()
            + "\","
            + "\"token\":"
            + ""
            + &self.token().to_string()
            + ","
            + "\"next\":"
            + &s_next
            + ","
            + "\"lhs\":"
            + &s_lhs
            + ","
            + "\"rhs\":"
            + &s_rhs
            + ","
            + "\"body\":"
            + &s_body
            + ","
            + "\"cond\":"
            + &s_cond
            + ","
            + "\"then\":"
            + &s_then
            + ","
            + "\"els\":"
            + &s_els
            + ","
            + "\"val\":"
            + &self.val().to_string()
            + ","
            + "\"init\":"
            + &s_init
            + ","
            + "\"inc\":"
            + &s_inc
            + ","
            + "\"args\":"
            + &s_args
            + ""
            + "}";
        return s;
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Member {
    next: MemberWrap,
    ty: TyWrap,
    name: TokenWrap,
    offset: i64,
}

#[allow(dead_code)]
impl Member {
    fn new() -> Self {
        Self {
            next: MemberWrap::empty(),
            ty: TyWrap::empty(),
            name: TokenWrap::empty(),
            offset: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MemberWrap {
    pub ptr: Option<*mut Member>,
}

#[allow(dead_code)]
impl Iterator for MemberWrap {
    type Item = MemberWrap;

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
impl MemberWrap {
    pub fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn new() -> Self {
        let mem: Option<*mut Member> = Some(Box::leak(Box::new(Member::new())));
        Self { ptr: mem }
    }

    pub fn ty(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().ty }
    }

    pub fn name(&self) -> TokenWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().name }
    }

    pub fn nxt(&self) -> MemberWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn offset(&self) -> i64 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().offset }
    }

    pub fn set_ty(&self, ty: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().ty = ty }
    }

    pub fn set_name(&self, name: TokenWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().name = name }
    }

    pub fn set_next(&self, next: MemberWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next }
    }

    pub fn set_offset(&self, offset: i64) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().offset = offset }
    }
}

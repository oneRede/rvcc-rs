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
    GOTO,
    LABEL,
    SWITCH,
    CASE,
    SHL,
    SHR,
    COND,
    NullExpr,
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
            NodeKind::GOTO => "GOTO".to_string(),
            NodeKind::LABEL => "LABEL".to_string(),
            NodeKind::SWITCH => "SWITCH".to_string(),
            NodeKind::CASE => "CASE".to_string(),
            NodeKind::SHL => "SHL".to_string(),
            NodeKind::SHR => "SHR".to_string(),
            NodeKind::COND => "COND".to_string(),
            NodeKind::NullExpr => "NullExpr".to_string(),
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

    pub label: &'static str,
    pub unique_label: &'static str,
    pub goto_next: NodeWrap,
    pub brk_label: &'static str,
    pub cont_label: &'static str,

    pub default_case: NodeWrap,
    pub case_next: NodeWrap,
}

#[allow(dead_code)]
impl Node {
    pub fn new() -> Self{
        Self{
            kind: NodeKind::ADDR,
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
            token: TokenWrap::empty(),
            ty: TyWrap::empty(),
            func_name: "",
            args: NodeWrap::empty(),
            mem: MemberWrap::empty(),
            func_type: TyWrap::empty(),
            label: "",
            unique_label: "",
            goto_next: NodeWrap::empty(),
            brk_label: ",",
            cont_label: "",

            default_case: NodeWrap::empty(),
            case_next: NodeWrap::empty(),
        }

    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeWrap {
    pub ptr: Option<*mut Node>,
}

#[allow(dead_code)]
impl NodeWrap {
    pub fn new(kind: NodeKind, token: TokenWrap) -> NodeWrap {
        let node = Node::new();
        let node: Option<*mut Node> = Some(Box::leak(Box::new(node)));
        let node = NodeWrap::new_node_wrap(node);
        node.set_kind(kind);
        node.set_token(token);
        node
    }

    pub fn new_binary(kind: NodeKind, lhs: NodeWrap, rhs: NodeWrap, token: TokenWrap) -> NodeWrap {
        let node = Node::new();
        let node: Option<*mut Node> = Some(Box::leak(Box::new(node)));
        let node = NodeWrap::new_node_wrap(node);
        node.set_kind(kind);
        node.set_lhs(lhs);
        node.set_rhs(rhs);
        node.set_token(token);
        node
    }

    pub fn new_num(val: i64, token: TokenWrap) -> NodeWrap {
        let node = Node::new();
        let node: Option<*mut Node> = Some(Box::leak(Box::new(node)));
        let node = NodeWrap::new_node_wrap(node);
        node.set_kind(NodeKind::Num);
        node.set_ty(TyWrap::new_with_kind(TypeKind::INT));
        node.set_val(val);
        node.set_token(token);
        node
    }

    pub fn new_long(val: i64, token: TokenWrap) -> NodeWrap {
        let node = Node::new();
        let node: Option<*mut Node> = Some(Box::leak(Box::new(node)));
        let node = NodeWrap::new_node_wrap(node);
        node.set_kind(NodeKind::Num);
        node.set_ty(TyWrap::new_with_kind(TypeKind::LONG));
        node.set_val(val);
        node.set_token(token);
        node
    }

    pub fn new_unary(kind: NodeKind, expr: NodeWrap, token: TokenWrap) -> NodeWrap {
        let node: NodeWrap = NodeWrap::new(kind, token);
        node.set_lhs(expr);
        return node;
    }

    pub fn new_var_node(var: ObjWrap, token: TokenWrap) -> NodeWrap {
        let node = Node::new();
        let node: Option<*mut Node> = Some(Box::leak(Box::new(node)));
        let node = NodeWrap::new_node_wrap(node);
        node.set_kind(NodeKind::VAR);
        node.set_var(var);
        node.set_token(token);
        node
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

    pub const fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn next_goto(&mut self) -> Option<NodeWrap> {
        let now = *self;
        if !now.ptr.is_none() {
            self.ptr = self.nxt_goto().ptr;
            return Some(now);
        } else {
            return None;
        }
    }

    #[inline]
    pub fn kind(&self) -> NodeKind {
        unsafe { self.ptr.unwrap().as_ref().unwrap().kind }
    }

    #[inline]
    pub fn nxt(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    #[inline]
    pub fn nxt_goto(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().goto_next }
    }

    #[inline]
    pub fn lhs(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().lhs }
    }

    #[inline]
    pub fn rhs(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().rhs }
    }

    #[inline]
    pub fn body(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().body }
    }

    pub fn cond(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().cond }
    }

    #[inline]
    pub fn then(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().then }
    }

    #[inline]
    pub fn els(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().els }
    }

    #[inline]
    pub fn val(&self) -> i64 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().val }
    }

    #[inline]
    pub fn var(&self) -> ObjWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().var }
    }

    #[inline]
    pub fn init(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().init }
    }

    #[inline]
    pub fn inc(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().inc }
    }

    #[inline]
    pub fn token(&self) -> TokenWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().token }
    }

    #[inline]
    pub fn ty(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().ty }
    }

    #[inline]
    pub fn func_name(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().func_name }
    }

    #[inline]
    pub fn args(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().args }
    }

    #[inline]
    pub fn mem(&self) -> MemberWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().mem }
    }

    #[inline]
    pub fn func_type(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().func_type }
    }

    #[inline]
    pub fn label(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().label }
    }

    #[inline]
    pub fn unique_label(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().unique_label }
    }

    #[inline]
    pub fn goto_next(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().goto_next }
    }

    #[inline]
    pub fn brk_label(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().brk_label }
    }

    #[inline]
    pub fn cont_label(&self) -> &'static str {
        unsafe { self.ptr.unwrap().as_ref().unwrap().cont_label }
    }

    #[inline]
    pub fn default_case(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().default_case }
    }

    #[inline]
    pub fn case_next(&self) -> NodeWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().case_next }
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

    pub fn set_label(&self, label: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().label = label }
    }

    pub fn set_unique_label(&self, unique_label: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().unique_label = unique_label }
    }

    pub fn set_goto_next(&self, goto_next: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().goto_next = goto_next }
    }

    pub fn set_brk_label(&self, brk_label: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().brk_label = brk_label }
    }

    pub fn set_cont_label(&self, cont_label: &'static str) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().cont_label = cont_label }
    }

    pub fn set_default_case(&self, default_case: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().default_case = default_case }
    }

    pub fn set_case_next(&self, case_next: NodeWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().case_next = case_next }
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
    token: TokenWrap,
}

#[allow(dead_code)]
impl Member {
    fn new() -> Self {
        Self {
            next: MemberWrap::empty(),
            ty: TyWrap::empty(),
            name: TokenWrap::empty(),
            offset: 0,
            token: TokenWrap::empty(),
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

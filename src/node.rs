use crate::{
    obj::ObjWrap,
    token::TokenWrap,
    ty::{TyWrap, TypeKind},
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

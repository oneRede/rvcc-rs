use crate::{parse::LOCALS, ty::create_ty};

#[allow(dead_code)]
pub static mut TYPE_INT: Option<*mut Ty> = None;

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
    pub next: Option<*mut Token>,
    pub val: i32,
    pub loc: Option<&'static [char]>,
    pub len: usize,
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_kind: TokenKind, loc: &'static [char], len: usize) -> Self {
        Self {
            kind: token_kind,
            next: None,
            val: 0,
            loc: Some(loc),
            len: len,
        }
    }
    pub fn empty() -> Self {
        Self {
            kind: TokenKind::EOF,
            next: None,
            val: 0,
            loc: None,
            len: 0,
        }
    }

    fn format(&self) -> String {
        if self.loc.is_none() {
            return "".to_string();
        }
        let loc: String = self.loc.unwrap()[..self.len].iter().collect();
        let mut _s = "".to_string();
        if self.next.is_none() {
            _s = "{".to_string()
                + "\"kind\":\""
                + &self.kind.to_string()
                + "\","
                + "\"val\":\""
                + &self.val.to_string()
                + "\","
                + "\"loc\":\""
                + &loc
                + "\","
                + "\"len\":\""
                + &self.len.to_string()
                + "\","
                + "\"next\": \"None\"}";
        } else {
            _s = "{".to_string()
                + "\"kind\":\""
                + &self.kind.to_string()
                + "\","
                + "\"val\":\""
                + &self.val.to_string()
                + "\","
                + "\"loc\":\""
                + &loc
                + "\","
                + "\"len\":\""
                + &self.len.to_string()
                + "\","
                + "\"next\":"
                + unsafe { &self.next.unwrap().as_ref().unwrap().format() }
                + "}";
        }
        _s
    }
}

impl Iterator for TokenWrap {
    type Item = *mut Token;

    fn next(&mut self) -> Option<Self::Item> {
        let now = self.ptr;
        if !now.is_none() {
            self.ptr = unsafe { self.ptr.unwrap().as_ref().unwrap().next };
            return now;
        } else {
            return None;
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.format()
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct TokenWrap {
    pub ptr: Option<*mut Token>,
}

#[allow(dead_code)]
impl TokenWrap {
    pub fn new(ptr: *mut Token) -> Self {
        Self { ptr: Some(ptr) }
    }

    pub fn empty() -> Self {
        Self {
            ptr: Some(Box::leak(Box::new(Token::empty()))),
        }
    }

    pub fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }

    pub fn set(&mut self, ptr: *mut Token) -> Self {
        self.ptr = Some(ptr);
        *self
    }

    pub fn set_next(self, next: *mut Token) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = Some(next) };
    }

    pub fn set_val(self, val: i32) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().val = val };
    }

    pub fn set_len(self, len: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().len = len };
    }

    pub fn get_next(&self) -> *mut Token {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next.unwrap() }
    }

    pub fn get_kind(&self) -> TokenKind {
        unsafe { self.ptr.unwrap().as_ref().unwrap().kind }
    }

    pub fn get_len(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().len }
    }

    pub fn get_val(&self) -> i32 {
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
pub struct Node {
    pub kind: NodeKind,

    pub next: Option<*mut Node>,

    pub lhs: Option<*mut Node>,
    pub rhs: Option<*mut Node>,

    pub body: Option<*mut Node>,

    pub cond: Option<*mut Node>,
    pub then: Option<*mut Node>,
    pub els: Option<*mut Node>,

    pub val: i64,
    pub var: Option<*mut Obj>,

    pub init: Option<*mut Node>,
    pub inc: Option<*mut Node>,

    pub token: TokenWrap,
    pub ty: Option<*mut Ty>,

    pub func_name: &'static str,
    pub args: Option<*mut Node>,
}

#[allow(dead_code)]
impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            kind: kind,
            next: None,
            lhs: None,
            rhs: None,
            body: None,
            cond: None,
            then: None,
            els: None,
            val: 0,
            var: None,
            init: None,
            inc: None,
            token: TokenWrap::empty(),
            ty: None,
            func_name: "",
            args: None,
        }
    }

    pub fn new_v2(kind: NodeKind, token: TokenWrap) -> Self {
        Self {
            kind: kind,
            next: None,
            lhs: None,
            rhs: None,
            body: None,
            cond: None,
            then: None,
            els: None,
            val: 0,
            var: None,
            init: None,
            inc: None,
            token: token,
            ty: None,
            func_name: "",
            args: None,
        }
    }

    pub fn new_binary(kind: NodeKind, lhs: *mut Node, rhs: *mut Node) -> Self {
        Self {
            kind: kind,
            next: None,
            lhs: Some(lhs),
            rhs: Some(rhs),
            body: None,
            cond: None,
            then: None,
            els: None,
            val: 0,
            var: None,
            init: None,
            inc: None,
            token: TokenWrap::empty(),
            ty: None,
            func_name: "",
            args: None,
        }
    }

    pub fn new_binary_v2(kind: NodeKind, lhs: *mut Node, rhs: *mut Node, token: TokenWrap) -> Self {
        Self {
            kind: kind,
            next: None,
            lhs: Some(lhs),
            rhs: Some(rhs),
            body: None,
            cond: None,
            then: None,
            els: None,
            val: 0,
            var: None,
            init: None,
            inc: None,
            token: token,
            ty: None,
            func_name: "",
            args: None,
        }
    }

    pub fn new_num(val: i64) -> Self {
        Self {
            kind: NodeKind::Num,
            next: None,
            lhs: None,
            rhs: None,
            body: None,
            cond: None,
            then: None,
            els: None,
            val: val,
            var: None,
            init: None,
            inc: None,
            token: TokenWrap::empty(),
            ty: None,
            func_name: "",
            args: None,
        }
    }

    pub fn new_num_v2(val: i64, token: TokenWrap) -> Self {
        Self {
            kind: NodeKind::Num,
            next: None,
            lhs: None,
            rhs: None,
            body: None,
            cond: None,
            then: None,
            els: None,
            val: val,
            var: None,
            init: None,
            inc: None,
            token: token,
            ty: create_ty(TypeKind::INT),
            func_name: "",
            args: None,
        }
    }

    pub fn new_unary(kind: NodeKind, expr: *mut Node) -> Self {
        let mut node: Node = Node::new(kind);
        node.lhs = Some(expr);
        return node;
    }

    pub fn new_unary_v2(kind: NodeKind, expr: Option<*mut Node>, token: TokenWrap) -> Self {
        let mut node: Node = Node::new_v2(kind, token);
        node.lhs = expr;
        return node;
    }

    pub fn new_var_node_v2(var: Option<*mut Obj>, token: TokenWrap) -> Self {
        Self {
            kind: NodeKind::VAR,
            next: None,
            lhs: None,
            rhs: None,
            body: None,
            cond: None,
            then: None,
            els: None,
            val: 0,
            var: var,
            init: None,
            inc: None,
            token: token,
            ty: None,
            func_name: "",
            args: None,
        }
    }

    fn format(&self) -> String {
        let mut _s_next = "".to_string();
        if self.next.is_none() {
            _s_next = "None".to_string();
        } else {
            _s_next = unsafe { self.next.unwrap().as_ref().unwrap().format() };
        }

        let mut _s_body = "".to_string();
        if self.body.is_none() {
            _s_body = "None".to_string();
        } else {
            _s_body = unsafe { self.body.unwrap().as_ref().unwrap().format() };
        }

        let mut _s_lhs = "".to_string();
        if self.lhs.is_none() {
            _s_lhs = "None".to_string();
        } else {
            _s_lhs = unsafe { self.lhs.unwrap().as_ref().unwrap().format() };
        }

        let mut _s_rhs = "".to_string();
        if self.rhs.is_none() {
            _s_rhs = "None".to_string();
        } else {
            _s_rhs = unsafe { self.rhs.unwrap().as_ref().unwrap().format() };
        }

        let mut _s_cond = "".to_string();
        if self.cond.is_none() {
            _s_cond = "None".to_string();
        } else {
            _s_cond = unsafe { self.cond.unwrap().as_ref().unwrap().format() };
        }

        let mut _s_then = "".to_string();
        if self.then.is_none() {
            _s_then = "None".to_string();
        } else {
            _s_then = unsafe { self.then.unwrap().as_ref().unwrap().format() };
        }

        let mut _s_els = "".to_string();
        if self.els.is_none() {
            _s_els = "None".to_string();
        } else {
            _s_els = unsafe { self.els.unwrap().as_ref().unwrap().format() };
        }

        let mut _s_init = "".to_string();
        if self.init.is_none() {
            _s_init = "None".to_string();
        } else {
            _s_init = unsafe { self.init.unwrap().as_ref().unwrap().format() };
        }

        let mut _s_inc = "".to_string();
        if self.inc.is_none() {
            _s_inc = "None".to_string();
        } else {
            _s_inc = unsafe { self.inc.unwrap().as_ref().unwrap().format() };
        }

        let mut _s_var = "".to_string();
        if self.var.is_none() {
            _s_var = "None".to_string();
        } else {
            _s_var = unsafe { self.var.unwrap().as_ref().unwrap().to_string() };
        }

        let mut _s_ty = "".to_string();
        if self.ty.is_none() {
            _s_ty = "None".to_string();
        } else {
            _s_ty = unsafe { self.ty.unwrap().as_ref().unwrap().to_string() };
        }

        let _s = "{".to_string()
            + "\"kind\":"
            + &self.kind.to_string()
            + ","
            + "\"next\":"
            + &_s_next
            + ","
            + "\"body\":"
            + &_s_body
            + ","
            + "\"lhs\":"
            + &_s_lhs
            + ","
            + "\"rhs\":"
            + &_s_rhs
            + ","
            + "\"cond\":"
            + &_s_cond
            + ","
            + "\"then\":"
            + &_s_then
            + ","
            + "\"els\":"
            + &_s_els
            + ","
            + "\"init\":"
            + &_s_init
            + ","
            + "\"inc\":"
            + &_s_inc
            + ","
            + "\"val\":"
            + &self.val.to_string()
            + ","
            + "\"var\":"
            + &_s_var
            + ","
            + "\"token\":"
            + &self.token.get_ref().to_string()
            + ","
            + "\"ty\":"
            + &_s_ty
            + "}";
        _s
    }
}

impl ToString for Node {
    fn to_string(&self) -> String {
        self.format()
    }
}

#[allow(dead_code)]
pub struct NodeIter {
    pub ptr: Option<*mut Node>,
}

impl Iterator for NodeIter {
    type Item = *mut Node;

    fn next(&mut self) -> Option<Self::Item> {
        let now = self.ptr;
        if !now.is_none() {
            self.ptr = unsafe { self.ptr.unwrap().as_ref().unwrap().next };
            return now;
        } else {
            return None;
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Obj {
    pub next: Option<*mut Obj>,
    pub name: &'static str,
    pub offset: i64,
    pub ty: Option<*mut Ty>,
}

#[allow(dead_code)]
impl Obj {
    pub fn new(name: &'static str, ty: Option<*mut Ty>) -> *mut Obj {
        let mut var = Self {
            next: None,
            name: name,
            offset: 0,
            ty: ty,
        };
        var.next = unsafe { LOCALS };
        let var: *mut Obj = Box::leak(Box::new(var));
        unsafe { LOCALS = Some(var) };
        var
    }
}

impl ToString for Obj {
    fn to_string(&self) -> String {
        let mut _s = "".to_string();
        if self.next.is_none() {
            _s = "{".to_string()
                + "\"name\":\""
                + self.name
                + "\","
                + "\"offset\":\""
                + &self.offset.to_string()
                + "\","
                + "\"next\": \"None\"}";
        } else {
            _s = "{".to_string()
                + "\"name\":\""
                + self.name
                + "\","
                + "\"offset\":\""
                + &self.offset.to_string()
                + "\","
                + "\"next\":"
                + unsafe { &self.next.unwrap().as_ref().unwrap().to_string() }
                + "}";
        }
        _s
    }
}

#[allow(dead_code)]
pub struct ObjIter {
    pub ptr: Option<*mut Obj>,
}

#[allow(dead_code)]
impl ObjIter {
    pub fn new(ptr: Option<*mut Obj>) -> Self {
        Self { ptr: ptr }
    }
}

#[allow(dead_code)]
impl Iterator for ObjIter {
    type Item = *mut Obj;

    fn next(&mut self) -> Option<Self::Item> {
        let now = self.ptr;
        if !now.is_none() {
            self.ptr = unsafe { self.ptr.unwrap().as_ref().unwrap().next };
            return now;
        } else {
            return None;
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Function {
    pub next: Option<*mut Function>,
    pub name: &'static str,
    pub body: Option<*mut Node>,
    pub locals: Option<*mut Obj>,
    pub stack_size: i64,
    pub params: Option<*mut Obj>
}


#[allow(dead_code)]
impl Function {
    pub fn new(body: *mut Node, locals: Option<*mut Obj>) -> Self {
        Self {
            next: None,
            name: "",
            body: Some(body),
            locals: locals,
            stack_size: 0,
            params: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            next: None,
            name: "",
            body: None,
            locals: None,
            stack_size: 0,
            params: None,
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TypeKind {
    INT,
    PTR,
    FUNC,
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Ty {
    pub kind: Option<TypeKind>,
    pub base: Option<*mut Ty>,
    pub token: TokenWrap,
    pub return_ty: Option<*mut Ty>,
    pub params: Option<*mut Ty>,
    pub next: Option<*mut Ty>,
}

#[allow(dead_code)]
impl Ty {
    pub fn new() -> Self {
        Self {
            kind: None,
            base: None,
            token: TokenWrap::empty(),
            return_ty: None,
            params: None,
            next: None,
        }
    }

    pub fn new_with_kind(kind: Option<TypeKind>) -> Self {
        Self {
            kind: kind,
            base: None,
            token: TokenWrap::empty(),
            return_ty: None,
            params: None,
            next: None,
        }
    }

    pub fn new_func_ty(return_ty: Option<*mut Ty>) -> Self {
        Self {
            kind: Some(TypeKind::FUNC),
            base: None,
            token: TokenWrap::empty(),
            return_ty: return_ty,
            params: None,
            next: None,
        }
    }

    pub fn point_to(base: Option<*mut Ty>) -> Self {
        Self {
            kind: Some(TypeKind::PTR),
            base: base,
            token: TokenWrap::empty(),
            return_ty: None,
            params: None,
            next: None,
        }
    }

    pub fn copy(&self) -> Option<*mut Ty>{
        let mut tmp = Ty::new();
        tmp.kind = self.kind;
        tmp.base = self.base;
        tmp.token = self.token;
        tmp.return_ty = self.return_ty;
        tmp.params=  self.params;
        tmp.next = self.next;

        Some(Box::leak(Box::new(tmp)))
    }
}

impl ToString for Ty {
    fn to_string(&self) -> String {
        match &self.kind.unwrap() {
            TypeKind::INT => "INT".to_string(),
            TypeKind::PTR => "PTR".to_string(),
            TypeKind::FUNC => "FUNC".to_string(),
        }
    }
}

#[allow(dead_code)]
pub fn get_token_ref(token: *mut Token) -> &'static Token {
    unsafe { token.as_ref().unwrap() }
}

#[allow(dead_code)]
pub fn get_node_kind(node: *mut Node) -> NodeKind {
    unsafe { node.as_ref().unwrap().kind }
}

#[allow(dead_code)]
pub fn set_node_kind(node: *mut Node, kind: NodeKind) {
    unsafe { node.as_mut().unwrap().kind = kind }
}

#[allow(dead_code)]
pub fn get_node_val(node: *mut Node) -> i64 {
    unsafe { node.as_ref().unwrap().val }
}

#[allow(dead_code)]
pub fn get_node_var(node: *mut Node) -> *mut Obj {
    unsafe { node.as_ref().unwrap().var.unwrap() }
}

#[allow(dead_code)]
pub fn get_node_lhs(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().lhs }
}

#[allow(dead_code)]
pub fn get_node_rhs(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().rhs }
}

#[allow(dead_code)]
pub fn get_node_next(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().next }
}

#[allow(dead_code)]
pub fn set_node_next(node: *mut Node, next: Option<*mut Node>) {
    unsafe { node.as_mut().unwrap().next = next }
}

#[allow(dead_code)]
pub fn get_node_body(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().body }
}

#[allow(dead_code)]
pub fn set_node_body(node: *mut Node, body: Option<*mut Node>) {
    unsafe { node.as_mut().unwrap().body = body }
}

#[allow(dead_code)]
pub fn get_node_cond(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().cond }
}

#[allow(dead_code)]
pub fn set_node_cond(node: *mut Node, cond: Option<*mut Node>) {
    unsafe { node.as_mut().unwrap().cond = cond }
}

#[allow(dead_code)]
pub fn get_node_then(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().then }
}

#[allow(dead_code)]
pub fn set_node_then(node: *mut Node, then: Option<*mut Node>) {
    unsafe { node.as_mut().unwrap().then = then }
}

#[allow(dead_code)]
pub fn get_node_els(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().els }
}

#[allow(dead_code)]
pub fn set_node_els(node: *mut Node, els: Option<*mut Node>) {
    unsafe { node.as_mut().unwrap().els = els }
}

#[allow(dead_code)]
pub fn get_node_init(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().init }
}

#[allow(dead_code)]
pub fn set_node_init(node: *mut Node, init: Option<*mut Node>) {
    unsafe { node.as_mut().unwrap().init = init }
}

#[allow(dead_code)]
pub fn get_node_inc(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().inc }
}

#[allow(dead_code)]
pub fn set_node_inc(node: *mut Node, inc: Option<*mut Node>) {
    unsafe { node.as_mut().unwrap().inc = inc }
}

#[allow(dead_code)]
pub fn get_node_token(node: *mut Node) -> TokenWrap {
    unsafe { node.as_ref().unwrap().token }
}

#[allow(dead_code)]
pub fn get_node_ty(node: *mut Node) -> Option<*mut Ty> {
    unsafe { node.as_ref().unwrap().ty }
}

#[allow(dead_code)]
pub fn set_node_ty(node: *mut Node, ty: Option<*mut Ty>) {
    unsafe { node.as_mut().unwrap().ty = ty }
}

#[allow(dead_code)]
pub fn get_node_func_name(node: *mut Node) -> &'static str {
    unsafe { node.as_ref().unwrap().func_name }
}

#[allow(dead_code)]
pub fn set_node_func_name(node: *mut Node, func_name: &'static str) {
    unsafe { node.as_mut().unwrap().func_name = func_name }
}

#[allow(dead_code)]
pub fn get_node_args(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().args }
}

#[allow(dead_code)]
pub fn set_node_args(node: *mut Node, args: Option<*mut Node>) {
    unsafe { node.as_mut().unwrap().args = args }
}

#[allow(dead_code)]
pub fn get_obj_next(obj: *mut Obj) -> Option<*mut Obj> {
    unsafe { obj.as_ref().unwrap().next }
}

#[allow(dead_code)]
pub fn get_obj_name(obj: *mut Obj) -> &'static str {
    unsafe { obj.as_ref().unwrap().name }
}

#[allow(dead_code)]
pub fn get_obj_offset(obj: *mut Obj) -> i64 {
    unsafe { obj.as_ref().unwrap().offset }
}

#[allow(dead_code)]
pub fn set_obj_offset(obj: *mut Obj, offset: i64) {
    unsafe { obj.as_mut().unwrap().offset = offset }
}

#[allow(dead_code)]
pub fn get_obj_ty(obj: *mut Obj) -> Option<*mut Ty> {
    unsafe { obj.as_ref().unwrap().ty }
}

#[allow(dead_code)]
pub fn get_function_locals(func: *mut Function) -> Option<*mut Obj> {
    unsafe { func.as_ref().unwrap().locals }
}

#[allow(dead_code)]
pub fn set_function_stack_size(func: *mut Function, stack_size: i64) {
    unsafe { func.as_mut().unwrap().stack_size = stack_size }
}

#[allow(dead_code)]
pub fn get_function_body(func: *mut Function) -> Option<*mut Node> {
    unsafe { func.as_ref().unwrap().body }
}

#[allow(dead_code)]
pub fn get_function_stack_size(func: *mut Function) -> i64 {
    unsafe { func.as_ref().unwrap().stack_size }
}

#[allow(dead_code)]
pub fn get_function_name(func: *mut Function) -> &'static str {
    unsafe { func.as_ref().unwrap().name }
}

#[allow(dead_code)]
pub fn get_function_next(func: *mut Function) -> Option<*mut Function> {
    unsafe { func.as_ref().unwrap().next }
}

#[allow(dead_code)]
pub fn set_function_next(func: *mut Function, next: Option<*mut Function>) {
    unsafe { func.as_mut().unwrap().next = next }
}

#[allow(dead_code)]
pub fn get_function_params(func: *mut Function) -> Option<*mut Obj> {
    unsafe { func.as_ref().unwrap().params }
}

#[allow(dead_code)]
pub fn get_ty_kind(ty: Option<*mut Ty>) -> Option<TypeKind> {
    if ty.is_none() {
        return None;
    }
    unsafe { ty.unwrap().as_ref().unwrap().kind }
}

#[allow(dead_code)]
pub fn get_ty_base(ty: *mut Ty) -> Option<*mut Ty> {
    unsafe { ty.as_ref().unwrap().base }
}

#[allow(dead_code)]
pub fn get_ty_ref(ty: Option<*mut Ty>) -> &'static Ty {
    if ty.is_none() {
        return Box::leak(Box::new(Ty::new_with_kind(Some(TypeKind::INT))));
    }
    unsafe { ty.unwrap().as_ref().unwrap() }
}

#[allow(dead_code)]
pub fn get_ty_token(ty: *mut Ty) -> TokenWrap {
    unsafe { ty.as_ref().unwrap().token }
}

#[allow(dead_code)]
pub fn set_ty_token(ty: *mut Ty, token: TokenWrap) {
    unsafe { ty.as_mut().unwrap().token = token }
}

#[allow(dead_code)]
pub fn get_ty_next(ty: *mut Ty) -> Option<*mut Ty> {
    unsafe { ty.as_ref().unwrap().next }
}

#[allow(dead_code)]
pub fn set_ty_next(ty: *mut Ty, next: Option<*mut Ty>) {
    unsafe { ty.as_mut().unwrap().next = next }
}

#[allow(dead_code)]
pub fn get_ty_params(ty: *mut Ty) -> Option<*mut Ty> {
    unsafe { ty.as_ref().unwrap().params }
}

#[allow(dead_code)]
pub fn set_ty_params(ty: *mut Ty, params: Option<*mut Ty>) {
    unsafe { ty.as_mut().unwrap().params = params }
}

#[test]
fn test_token_display() {
    let mut t1 = Token::new(TokenKind::Num, &['1'], 1);
    let t2 = Token::new(TokenKind::Num, &['2'], 1);
    t1.next = Some(Box::leak(Box::new(t2)));
    println!("{}", t1.to_string());
}

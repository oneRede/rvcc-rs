use crate::parse::LOCALS;

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
#[derive(Clone, Copy)]
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
    pub val: i64,
    pub var: Option<*mut Obj>,
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
            val: 0,
            var: None,
        }
    }

    pub fn new_binary(kind: NodeKind, lhs: *mut Node, rhs: *mut Node) -> Self {
        Self {
            kind: kind,
            next: None,
            lhs: Some(lhs),
            rhs: Some(rhs),
            body: None,
            val: 0,
            var: None,
        }
    }

    pub fn new_num(val: i64) -> Self {
        Self {
            kind: NodeKind::Num,
            next: None,
            lhs: None,
            rhs: None,
            body: None,
            val: val,
            var: None,
        }
    }

    pub fn new_unary(kind: NodeKind, expr: *mut Node) -> Self {
        let mut node: Node = Node::new(kind);
        node.lhs = Some(expr);
        return node;
    }

    pub fn new_var_node(var: Option<*mut Obj>) -> Self {
        Self {
            kind: NodeKind::VAR,
            next: None,
            lhs: None,
            rhs: None,
            body: None,
            val: 0,
            var: var,
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

        let mut _s_var = "".to_string();
        if self.var.is_none() {
            _s_var = "None".to_string();
        } else {
            _s_var = unsafe { self.var.unwrap().as_ref().unwrap().to_string() };
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
            + "\"val\":"
            + &self.val.to_string()
            + ","
            + "\"var\":"
            + &_s_var
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
}

#[allow(dead_code)]
impl Obj {
    pub fn new(name: &'static str) -> *mut Obj {
        let mut var = Self {
            next: None,
            name: name,
            offset: 0,
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
    pub body: Option<*mut Node>,
    pub locals: Option<*mut Obj>,
    pub stack_size: i64,
}

#[allow(dead_code)]
impl Function {
    pub fn new(body: *mut Node, locals: Option<*mut Obj>) -> Self {
        Self {
            body: Some(body),
            locals: locals,
            stack_size: 0,
        }
    }

    pub fn empty() -> Self {
        Self {
            body: None,
            locals: None,
            stack_size: 0,
        }
    }
}

#[allow(dead_code)]
pub fn get_node_kind(node: *mut Node) -> NodeKind {
    unsafe { node.as_ref().unwrap().kind }
}

#[allow(dead_code)]
pub fn get_node_val(node: *mut Node) -> i64 {
    unsafe { node.as_ref().unwrap().val }
}

#[allow(dead_code)]
pub fn get_node_lhs(node: *mut Node) -> *mut Node {
    unsafe { node.as_ref().unwrap().lhs.unwrap() }
}

#[allow(dead_code)]
pub fn get_node_rhs(node: *mut Node) -> *mut Node {
    unsafe { node.as_ref().unwrap().rhs.unwrap() }
}

#[allow(dead_code)]
pub fn get_node_next(node: *mut Node) -> Option<*mut Node> {
    unsafe { node.as_ref().unwrap().next }
}

#[allow(dead_code)]
pub fn get_obj_next(obj: *mut Obj) -> Option<*mut Obj> {
    unsafe { obj.as_ref().unwrap().next }
}

#[allow(dead_code)]
pub fn get_obj_name(obj: *mut Obj) -> &'static str {
    unsafe { obj.as_ref().unwrap().name }
}

#[test]
fn test_token_display() {
    let mut t1 = Token::new(TokenKind::Num, &['1'], 1);
    let t2 = Token::new(TokenKind::Num, &['2'], 1);
    t1.next = Some(Box::leak(Box::new(t2)));
    println!("{}", t1.to_string());
}

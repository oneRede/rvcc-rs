use crate::parse::LOCALS;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    IDENT,
    Punct,
    Num,
    Eof,
}

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Punct => "Punct".to_string(),
            TokenKind::IDENT => "IDENT".to_string(),
            TokenKind::Num => "Num".to_string(),
            TokenKind::Eof => "EOF".to_string(),
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
            kind: TokenKind::Eof,
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

impl ToString for Token {
    fn to_string(&self) -> String {
        self.format()
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
}

impl ToString for NodeKind{
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
            val: 0,
            var: var,
        }
    }

    fn format(&self) -> String {
        let mut s_next = "".to_string();
        if self.next.is_none(){
            s_next = "None".to_string();
        } else {
            s_next = unsafe { self.next.unwrap().as_ref().unwrap().format() };
        }

        let mut s_lhs = "".to_string();
        if self.lhs.is_none(){
            s_lhs = "None".to_string();
        } else {
            s_lhs = unsafe { self.lhs.unwrap().as_ref().unwrap().format() };
        }

        let mut s_rhs = "".to_string();
        if self.rhs.is_none(){
            s_rhs = "None".to_string();
        } else {
            s_rhs = unsafe { self.rhs.unwrap().as_ref().unwrap().format() };
        }

        let mut s_var = "".to_string();
        if self.var.is_none(){
            s_var = "None".to_string();
        } else {
            s_var = unsafe { self.var.unwrap().as_ref().unwrap().to_string() };
        }

        let _s = "{".to_string()
                + "\"kind\":\""
                + &self.kind.to_string()
                + "\","
                + "\"next\":\""
                + &s_next
                + "\","
                + "\"lhs\":\""
                + &s_lhs
                + "\","
                + "\"rhs\":\""
                + &s_rhs
                + "\","
                + "\"val\":\""
                + &self.val.to_string()
                + "\","
                + "\"var\":"
                + &s_var
                + "}";
        _s
    }
}

impl ToString for Node{
    fn to_string(&self) -> String {
        self.format()
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
#[derive(Clone, Copy, Debug)]
pub struct Function {
    pub body: *mut Node,
    pub locals: Option<*mut Obj>,
    pub stack_size: i64,
}

impl Function {
    pub fn new(body: *mut Node, locals: Option<*mut Obj>) -> Self {
        Self {
            body: body,
            locals: locals,
            stack_size: 0,
        }
    }
}

#[test]
fn test_token_display() {
    let mut t1 = Token::new(TokenKind::Num, &['1'], 1);
    let t2 = Token::new(TokenKind::Num, &['2'], 1);
    t1.next = Some(Box::leak(Box::new(t2)));
    println!("{}", t1.to_string());
}

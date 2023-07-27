use crate::parse::LOCALS;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum TokenKind {
    IDENT,
    Punct,
    Num,
    Eof,
}

#[allow(dead_code)]
pub struct Token {
    pub kind: TokenKind,
    pub next: Option<*mut Token>,
    pub val: i32,
    pub loc: Option<&'static [char]>,
    pub len: usize,
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
}

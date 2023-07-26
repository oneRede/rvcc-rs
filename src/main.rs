use std::{env, process::exit, slice};
mod utils;
use utils::get_num_from_chars;

static mut CURRENT_INPUT: Option<&[char]> = None;
static mut CURRENT_STR: Option<&str> = None;
static mut DEPTH: usize = 0;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
enum TokenKind {
    Punct,
    Num,
    Eof,
}

#[allow(dead_code)]
struct Token {
    kind: TokenKind,
    next: Option<*mut Token>,
    val: i32,
    loc: Option<&'static [char]>,
    len: usize,
}

#[allow(dead_code)]
impl Token {
    fn new(token_kind: TokenKind, loc: &'static [char], len: usize) -> Self {
        Self {
            kind: token_kind,
            next: None,
            val: 0,
            loc: Some(loc),
            len: len,
        }
    }
    fn empty() -> Self {
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
fn equal(token: &Token, s: &[char]) -> bool {
    if token.loc.unwrap().len() == 0 {
        return false;
    }
    if s.starts_with(unsafe { slice::from_raw_parts(token.loc.unwrap().as_ptr(), token.len) }) {
        return true;
    } else {
        return false;
    }
}

#[allow(dead_code)]
fn skip<'a>(token: &Token, s: &[char]) -> Option<*mut Token> {
    if !equal(&token, s) {
        error_token(token, &format!("expect {:?}", s));
    }
    return token.next;
}

#[allow(dead_code)]
fn get_num(token: &Token) -> i32 {
    if token.kind != TokenKind::Num {
        error_token(token, "expect a num");
    }
    token.val
}

#[allow(dead_code)]
fn tokenize(mut chars: &'static [char]) -> Option<*mut Token> {
    let head: *mut Token = &mut Token::empty() as *mut Token;
    let mut cur = head;

    loop {
        if chars.len() == 0 {
            unsafe {
                cur.as_mut().unwrap().next =
                    Some(Box::leak(Box::new(Token::new(TokenKind::Eof, chars, 0))))
            };
            return unsafe { head.as_ref().unwrap().next };
        }

        let c: char = chars[0];
        if c.is_whitespace() {
            chars = &chars[1..];
            continue;
        }

        let num_rs = get_num_from_chars(chars);
        if let Ok((num, cs)) = num_rs {
            unsafe {
                cur.as_mut().unwrap().next = Some(Box::leak(Box::new(Token::new(
                    TokenKind::Num,
                    chars,
                    num.to_string().len(),
                ))));
            }
            chars = cs;
            cur = unsafe { cur.as_ref().unwrap().next.unwrap() };
            unsafe { cur.as_mut().unwrap().val = num };
            unsafe { cur.as_mut().unwrap().len = num.to_string().len() };
            continue;
        }

        let len_punct = read_punct(chars);
        if len_punct > 0 {
            unsafe {
                cur.as_mut().unwrap().next = Some(Box::leak(Box::new(Token::new(
                    TokenKind::Punct,
                    chars,
                    len_punct,
                ))))
            };
            cur = unsafe { cur.as_mut().unwrap().next.unwrap() };
            chars = &chars[len_punct..];
            continue;
        }
        error_at(chars.as_ptr(), &format!("invalid token: {}", chars[0]));
    }
}

#[allow(dead_code)]
fn v_error_at(loc: *const char, msg: &str) {
    let input = unsafe { CURRENT_STR.unwrap() };
    let chars = unsafe { CURRENT_INPUT.unwrap() };
    eprintln!("{:?}", input);
    let distance = (unsafe { loc.offset_from(chars.as_ptr()) }).abs() - 1;
    eprintln!("{}", distance);
    eprint!("{:?}", " ".repeat(distance as usize));
    eprint!("{}", "^");
    eprintln!("{}", msg);
}

#[allow(dead_code)]
fn error_at(loc: *const char, msg: &str) {
    v_error_at(loc, msg);
    exit(1);
}

#[allow(dead_code)]
fn error_token(token: &Token, msg: &str) {
    let loc = token.loc.unwrap().as_ptr();
    v_error_at(loc, msg);
    exit(1);
}

#[allow(dead_code)]
fn read_punct(ptr: &[char]) -> usize {
    if starts_with(ptr, &['=', '='])
        || starts_with(ptr, &['!', '='])
        || starts_with(ptr, &['<', '='])
        || starts_with(ptr, &['>', '='])
    {
        return 2;
    }
    if ptr[0] == '+'
        || ptr[0] == '-'
        || ptr[0] == '*'
        || ptr[0] == '/'
        || ptr[0] == '('
        || ptr[0] == ')'
        || ptr[0] == '>'
        || ptr[0] == '<'
        || ptr[0] == '='
        || ptr[0] == '!'
    {
        return 1;
    } else {
        return 0;
    }
}

#[allow(dead_code)]
fn starts_with(s_str: &[char], sub_str: &[char]) -> bool {
    for i in 0..sub_str.len() {
        if s_str[i] != sub_str[i] {
            return false;
        }
    }

    true
}

#[allow(dead_code)]
fn expr(mut _rest: *mut *mut Token, token: *mut Token) -> *mut Node {
    return equality(_rest, token);
}

#[allow(dead_code)]
fn equality(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node: *mut Node = relational(&mut token, token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['=', '=']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::EQ,
                node,
                relational(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
            )));
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['!', '=']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::NE,
                node,
                relational(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
            )));
            continue;
        }

        unsafe { *_rest = token };
        return node;
    }
}

#[allow(dead_code)]
fn relational(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node = add(&mut token, token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['<']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::LT,
                node,
                add(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
            )));
            continue;
        }

        if equal(unsafe { token.as_ref().unwrap() }, &['<', '=']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::LE,
                node,
                add(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
            )));
            continue;
        }

        if equal(unsafe { token.as_ref().unwrap() }, &['>']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::LT,
                add(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
                node,
            )));
            continue;
        }

        if equal(unsafe { token.as_ref().unwrap() }, &['>', '=']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::LE,
                add(&mut token, unsafe { token.as_ref().unwrap().next.unwrap() }),
                node,
            )));
            continue;
        }

        unsafe { *_rest = token };
        return node;
    }
}

#[allow(dead_code)]
fn add(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node = mul(&mut token as *mut *mut Token, token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['+']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::Add,
                node,
                mul(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                }),
            )));
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['-']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::Sub,
                node,
                mul(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                }),
            )));
            continue;
        }
        unsafe { *_rest = token };
        return node;
    }
}

#[allow(dead_code)]
fn mul(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node = unary(&mut token as *mut *mut Token, token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['*']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::Mul,
                node,
                unary(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                }),
            )));
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['/']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::Div,
                node,
                unary(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                }),
            )));
            continue;
        }
        unsafe { *_rest = token };
        return node;
    }
}

#[allow(dead_code)]
fn unary(mut _rest: *mut *mut Token, token: *mut Token) -> *mut Node {
    if equal(unsafe { token.as_ref().unwrap() }, &['+']) {
        return unary(_rest, unsafe { token.as_ref().unwrap().next.unwrap() });
    }
    if equal(unsafe { token.as_ref().unwrap() }, &['-']) {
        return Box::leak(Box::new(Node::new_unary(
            NodeKind::NEG,
            unary(_rest, unsafe { token.as_ref().unwrap().next.unwrap() }),
        )));
    }
    primary(_rest, token).unwrap()
}

#[allow(dead_code)]
fn primary(mut _rest: *mut *mut Token, mut token: *mut Token) -> Option<*mut Node> {
    if equal(unsafe { token.as_ref().unwrap() }, &['(']) {
        let node = expr(
            &mut token as *mut *mut Token,
            unsafe { token.as_ref().unwrap().next }.unwrap(),
        );
        unsafe { *_rest = token.as_ref().unwrap().next.unwrap() };
        return Some(node);
    }

    if (unsafe { token.as_ref().unwrap().kind } == TokenKind::Num) {
        let node = Box::leak(Box::new(Node::new_num(
            unsafe { token.as_ref().unwrap().val } as i64,
        )));
        unsafe { *_rest = token.as_ref().unwrap().next.unwrap() };
        return Some(node);
    }

    error_token(unsafe { token.as_ref().unwrap() }, "expected an expression");
    None
}

#[allow(dead_code)]
fn push() {
    println!("  addi sp, sp, -8");
    println!("  sd a0, 0(sp)");
    unsafe { DEPTH += 1 };
}

#[allow(dead_code)]
fn pop(reg: &str) {
    println!("  ld {}, 0(sp)", reg);
    println!("  addi sp, sp, 8");
    unsafe { DEPTH -= 1 };
}

#[allow(dead_code)]
fn gen_expr(node: *mut Node) {
    match unsafe { node.as_ref().unwrap().kind } {
        NodeKind::Num => {
            println!("  li a0, {:?}", unsafe { node.as_ref().unwrap().val });
            return;
        }
        NodeKind::NEG => {
            gen_expr(unsafe { node.as_ref().unwrap().lhs }.unwrap());
            println!("  neg a0, a0");
            return;
        }
        _ => {}
    }

    gen_expr(unsafe { node.as_ref().unwrap().rhs }.unwrap());
    push();
    gen_expr(unsafe { node.as_ref().unwrap().lhs }.unwrap());
    pop("a1");

    match unsafe { node.as_ref().unwrap().kind } {
        NodeKind::Add => {
            println!("  add a0, a0, a1");
            return;
        }
        NodeKind::Sub => {
            println!("  sub a0, a0, a1");
            return;
        }
        NodeKind::Mul => {
            println!("  mul a0, a0, a1");
            return;
        }
        NodeKind::Div => {
            println!("  div a0, a0, a1");
            return;
        }
        NodeKind::EQ => {
            println!("  xor a0, a0, a1");
            println!("  seqz a0, a0");
            return;
        }
        NodeKind::NE => {
            println!("  xor a0, a0, a1");
            println!("  snez a0, a0");
            return;
        }
        NodeKind::LT => {
            println!("  slt a0, a0, a1");
            return;
        }
        NodeKind::LE => {
            println!("  slt a0, a1, a0");
            println!("  xori a0, a0, 1");
            return;
        }
        _ => {
            return;
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum NodeKind {
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
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct Node {
    kind: NodeKind,
    lhs: Option<*mut Node>,
    rhs: Option<*mut Node>,
    val: i64,
}

#[allow(dead_code)]
impl Node {
    fn new(kind: NodeKind) -> Self {
        Self {
            kind: kind,
            lhs: None,
            rhs: None,
            val: 0,
        }
    }

    fn new_binary(kind: NodeKind, lhs: *mut Node, rhs: *mut Node) -> Self {
        Self {
            kind: kind,
            lhs: Some(lhs),
            rhs: Some(rhs),
            val: 0,
        }
    }

    fn new_num(val: i64) -> Self {
        Self {
            kind: NodeKind::Num,
            lhs: None,
            rhs: None,
            val: val,
        }
    }

    fn new_unary(kind: NodeKind, expr: *mut Node) -> Self {
        let mut node: Node = Node::new(kind);
        node.lhs = Some(expr);
        return node;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if &args.len() != &2 {
        println!("{}: invalid number of arguments\n", &args.get(0).unwrap());
        exit(1)
    }

    let input: &str = Box::leak(Box::new(String::from(&args[1])));
    let chars: Vec<char> = input.chars().collect();
    let chars: &[char] = Box::leak(Box::new(chars));

    unsafe { CURRENT_STR = Some(input) };
    unsafe { CURRENT_INPUT = Some(chars) };

    let mut token = tokenize(chars).unwrap();
    let node = expr(&mut token as *mut *mut Token, token);

    if unsafe { token.as_ref().unwrap().kind } != TokenKind::Eof {
        error_token(unsafe { token.as_ref().unwrap() }, "extra token");
    }

    println!("  .globl main");
    println!("main:");
    gen_expr(node);
    println!("  ret");
    assert!(unsafe { DEPTH == 0 });
    return;
}

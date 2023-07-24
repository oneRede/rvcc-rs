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
    if token.loc.unwrap().len() == 0{
        return false
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
            chars = cs;

            unsafe {
                cur.as_mut().unwrap().next = Some(Box::leak(Box::new(Token::new(
                    TokenKind::Num,
                    chars,
                    num.to_string().len(),
                ))));
            }
            cur = unsafe { cur.as_ref().unwrap().next.unwrap() };
            unsafe { cur.as_mut().unwrap().val = num };
            unsafe { cur.as_mut().unwrap().len = num.to_string().len() };
            continue;
        }

        if chars[0] == '+'
            || chars[0] == '-'
            || chars[0] == '*'
            || chars[0] == '/'
            || chars[0] == '('
            || chars[0] == ')'
        {
            unsafe {
                cur.as_mut().unwrap().next =
                    Some(Box::leak(Box::new(Token::new(TokenKind::Punct, chars, 1))))
            };
            cur = unsafe { cur.as_mut().unwrap().next.unwrap() };
            chars = &chars[1..];
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
    // eprint!("{:?}", " ".repeat(distance as usize));
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
fn expr(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node = mul(&mut token as *mut *mut Token, token);

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['+']) {
            node = &mut Node::new_binary(
                NodeKind::Add,
                node,
                mul(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                }),
            ) as *mut Node;
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['-']) {
            node = &mut Node::new_binary(
                NodeKind::Sub,
                node,
                mul(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                }),
            ) as *mut Node;
            continue;
        }
        unsafe { *_rest = token};
        return node;
    }
}

#[allow(dead_code)]
fn mul(mut _rest: *mut *mut Token, mut token: *mut Token) -> *mut Node {
    let mut node = primary(&mut token as *mut *mut Token, token).unwrap();

    loop {
        if equal(unsafe { token.as_ref().unwrap() }, &['*']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::Mul,
                node,
                primary(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                })
                .unwrap(),
            )));
            continue;
        }
        if equal(unsafe { token.as_ref().unwrap() }, &['/']) {
            node = Box::leak(Box::new(Node::new_binary(
                NodeKind::Div,
                node,
                primary(&mut token as *mut *mut Token, unsafe {
                    token.as_ref().unwrap().next.unwrap()
                })
                .unwrap(),
            )));
            continue;
        }
        unsafe { *_rest = token};
        return node;
    }
}

#[allow(dead_code)]
fn primary(mut _rest: *mut *mut Token, mut token: *mut Token) -> Option<*mut Node> {
    if equal(unsafe { token.as_ref().unwrap() }, &['(']) {
        let node = expr(
            &mut token as *mut *mut Token,
            unsafe { token.as_ref().unwrap().next }.unwrap(),
        );
        unsafe { *_rest = token.as_ref().unwrap().next.unwrap()};
        return Some(node);
    }

    if (unsafe { token.as_ref().unwrap().kind } == TokenKind::Num) {
        let node = Node::new_num(unsafe { token.as_ref().unwrap().val } as i64);
        unsafe { *_rest = token.as_ref().unwrap().next.unwrap()};
        return Some(Box::leak(Box::new(node)));
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
    if unsafe { node.as_ref().unwrap().kind } == NodeKind::Num {
        println!("  li a0, {:?}", unsafe { node.as_ref().unwrap() });
        return;
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

    println!("token val: {:?}", unsafe{token.as_ref().unwrap().val});
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

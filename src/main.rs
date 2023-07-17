use std::{env, process::exit, slice};
mod utils;

use utils::get_num_from_chars;

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
    if s.starts_with(unsafe { slice::from_raw_parts(token.loc.unwrap().as_ptr(), token.len) }) {
        return true;
    } else {
        return false;
    }
}

#[allow(dead_code)]
fn skip<'a>(token: &Token, s: &[char]) -> Option<*mut Token> {
    if !equal(&token, s) {
        println!("expect '{:?}'", s)
    }
    return token.next;
}

#[allow(dead_code)]
fn get_num(token: &Token) -> i32 {
    if token.kind != TokenKind::Num {
        println!("expect a num")
    }
    token.val
}

#[allow(dead_code)]
fn tokenize(mut chars: &'static [char]) -> Option<*mut Token> {
    let head: *mut Token = Box::leak(Box::new(Token::empty()));
    let mut cur = head;

    loop {
        if chars.len() == 0 {
            unsafe {
                cur.as_mut().unwrap().next = Some(Box::leak(Box::new(Token::new(TokenKind::Eof, chars, 0))))
            };
            return unsafe { head.as_mut().unwrap().next }
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
            cur = unsafe { cur.as_mut().unwrap().next.unwrap() };
            unsafe { cur.as_mut().unwrap().val = num };
            unsafe { cur.as_mut().unwrap().len = num.to_string().len() };
            continue;
        }

        if chars[0] == '+' || chars[0] == '-' {
            unsafe {
                cur.as_mut().unwrap().next =
                    Some(Box::leak(Box::new(Token::new(TokenKind::Punct, chars, 1))))
            };
            cur = unsafe { cur.as_mut().unwrap().next.unwrap() };
            chars = &chars[1..];
            continue;
        }
        println!("invalid token: {}", chars[0]);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if &args.len() != &2 {
        println!("{}: invalid number of arguments\n", &args.get(0).unwrap());
        exit(1)
    }
    let chars: Vec<char> = args[1].chars().collect();
    let chars: &[char] = Box::leak(Box::new(chars));
    let mut token = tokenize(chars);

    println!("  .globl main");
    println!("main:");
    println!(
        "  li a0, {}",
        get_num(unsafe { token.unwrap().as_ref().unwrap() })
    );
    token = unsafe { token.unwrap().as_ref().unwrap().next };
    loop {
        if unsafe { token.unwrap().as_ref().unwrap().kind } == TokenKind::Eof {
            break;
        }
        if equal(unsafe { token.unwrap().as_ref().unwrap() }, &['+']) {
            token = unsafe { token.unwrap().as_ref().unwrap().next };
            println!(
                "  addi a0, a0, {}",
                get_num(unsafe { token.unwrap().as_ref().unwrap() })
            );
            token = unsafe { token.unwrap().as_ref().unwrap().next };
            continue;
        }

        token = skip(unsafe { token.unwrap().as_ref().unwrap() }, &['-']);
        println!(
            "  addi a0, a0, -{}",
            get_num(unsafe { token.unwrap().as_ref().unwrap() })
        );
        token = unsafe { token.unwrap().as_ref().unwrap().next };
    }

    println!("  ret");
    exit(0)
}

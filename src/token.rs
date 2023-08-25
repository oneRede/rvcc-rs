use std::{slice, str};

use self::TokenKind::*;
use crate::utils::{error_at, error_token, get_num_from_chars, read_punct};

pub static mut CURRENT_INPUT: Option<&[char]> = None;
pub static mut CURRENT_STR: Option<&str> = None;

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
    pub next: TokenWrap,
    pub val: i32,
    pub loc: Option<&'static [char]>,
    pub len: usize,
}

#[allow(dead_code)]
impl Iterator for TokenWrap {
    type Item = TokenWrap;

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
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct TokenWrap {
    pub ptr: Option<*mut Token>,
}

#[allow(dead_code)]
impl TokenWrap {
    pub fn new(token_kind: TokenKind, loc: &'static [char], len: usize) -> Self {
        let tk = Token {
            kind: token_kind,
            next: TokenWrap::empty(),
            val: 0,
            loc: Some(loc),
            len: len,
        };
        let tk: Option<*mut Token> = Some(Box::leak(Box::new(tk)));
        Self { ptr: tk }
    }

    pub fn init() -> Self {
        let tk = Token {
            kind: TokenKind::Num,
            next: TokenWrap::empty(),
            val: 0,
            loc: None,
            len: 0,
        };
        let tk: Option<*mut Token> = Some(Box::leak(Box::new(tk)));
        Self { ptr: tk }
    }

    pub fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn set_next(self, next: TokenWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next };
    }

    pub fn set_kind(self, kind: TokenKind) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().kind = kind };
    }

    pub fn set_val(self, val: i32) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().val = val };
    }

    pub fn set_len(self, len: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().len = len };
    }

    pub fn nxt(&self) -> TokenWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn kind(&self) -> TokenKind {
        unsafe { self.ptr.unwrap().as_ref().unwrap().kind }
    }

    pub fn get_len(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().len }
    }

    pub fn val(&self) -> i32 {
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
pub fn equal(token: TokenWrap, s: &str) -> bool {
    let token = token.get_ref();
    let s = str_to_chars(s);
    if token.len != s.len() {
        return false;
    }
    if unsafe { slice::from_raw_parts(token.loc.unwrap().as_ptr(), token.len) }.starts_with(s) {
        return true;
    } else {
        return false;
    }
}

#[allow(dead_code)]
pub fn str_to_chars(s: &str) -> &[char] {
    Box::leak(Box::new(
        s.chars().map(|c| -> char { c }).collect::<Vec<char>>(),
    ))
}

#[allow(dead_code)]
pub fn skip<'a>(token: TokenWrap, s: &str) -> TokenWrap {
    if !equal(token, s) {
        error_token(token, &format!("expect {:?}", s));
    }
    return token.nxt();
}

#[allow(dead_code)]
pub fn get_num(token: TokenWrap) -> i32 {
    if token.kind() != Num {
        error_token(token, "expect a num");
    }
    token.val()
}

#[allow(dead_code)]
pub fn tokenize(mut chars: &'static [char]) -> TokenWrap {
    let mut head: TokenWrap = TokenWrap::init();
    let mut cur = head;

    loop {
        if chars.len() == 0 {
            cur.set_next(TokenWrap::new(EOF, chars, 0));

            head = head.nxt();
            convert_keyword(head);
            return head;
        }

        let c: char = chars[0];
        if c.is_whitespace() {
            chars = &chars[1..];
            continue;
        }

        let num_rs = get_num_from_chars(chars);
        if let Ok((num, cs)) = num_rs {
            cur.set_next(TokenWrap::new(Num, chars, num.to_string().len()));

            chars = cs;
            cur = cur.nxt();
            cur.set_val(num);
            cur.set_len(num.to_string().len());
            continue;
        }

        if is_ident_v1(chars[0]) {
            let mut len_ident = 1;

            loop {
                if is_ident_v2(chars[len_ident]) {
                    len_ident += 1;
                } else {
                    break;
                }
            }

            cur.set_next(TokenWrap::new(IDENT, chars, len_ident));
            cur = cur.nxt();
            chars = &chars[len_ident..];
            continue;
        }

        match chars[0] {
            'a'..='z' => {
                cur.set_next(TokenWrap::new(IDENT, chars, 1));
                cur = cur.nxt();
                chars = &chars[1..];
                continue;
            }
            _ => {}
        }

        let len_punct = read_punct(chars);
        if len_punct > 0 {
            cur.set_next(TokenWrap::new(Punct, chars, len_punct));
            cur = cur.nxt();
            chars = &chars[len_punct..];
            continue;
        }

        error_at(chars.as_ptr(), &format!("invalid token: {}", chars[0]));
    }
}

#[allow(dead_code)]
pub fn is_ident_v1(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => {
            return true;
        }
        _ => return false,
    }
}

#[allow(dead_code)]
pub fn is_ident_v2(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => return true,
        _ => return false,
    }
}

#[allow(dead_code)]
pub fn convert_keyword(token: TokenWrap) {
    for tk in token {
        if is_keyword(tk) {
            tk.set_kind(KEYWORD);
            return;
        }
    }
}

#[allow(dead_code)]
fn is_keyword(token: TokenWrap) -> bool {
    let keywords = ["return", "if", "else", "for", "while", "int", "sizeof", "char"];

    for kw in keywords {
        if equal(token, kw) {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
pub fn consume(token: TokenWrap, s: &str) -> (bool, TokenWrap) {
    if equal(token, s) {
        return (true, token.nxt());
    }
    return (false, token);
}

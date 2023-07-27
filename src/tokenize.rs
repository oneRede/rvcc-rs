use std::{process::exit, slice};

use crate::{
    rvcc::{Token, TokenKind},
    utils::{error_at, get_num_from_chars, read_punct, v_error_at},
};

pub static mut CURRENT_INPUT: Option<&[char]> = None;
pub static mut CURRENT_STR: Option<&str> = None;

#[allow(dead_code)]
pub fn equal(token: &Token, s: &[char]) -> bool {
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
pub fn error_token(token: &Token, msg: &str) {
    let loc = token.loc.unwrap().as_ptr();
    v_error_at(loc, msg);
    exit(1);
}

#[allow(dead_code)]
pub fn skip<'a>(token: &Token, s: &[char]) -> Option<*mut Token> {
    if !equal(&token, s) {
        error_token(token, &format!("expect {:?}", s));
    }
    return token.next;
}

#[allow(dead_code)]
pub fn get_num(token: &Token) -> i32 {
    if token.kind != TokenKind::Num {
        error_token(token, "expect a num");
    }
    token.val
}

#[allow(dead_code)]
pub fn tokenize(mut chars: &'static [char]) -> Option<*mut Token> {
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

        if is_ident_v1(chars[0]) {
            let mut len_ident = 1;
            
            loop {
                if is_ident_v2(chars[len_ident]){
                    len_ident += 1;
                } else{
                    break;
                }
            }
            unsafe {
                cur.as_mut().unwrap().next =
                    Some(Box::leak(Box::new(Token::new(TokenKind::IDENT, chars, len_ident))));
            }
            cur = unsafe { cur.as_ref().unwrap().next.unwrap() };
            chars = &chars[len_ident..];
            continue;
        }

        match chars[0] {
            'a'..='z' => {
                unsafe {
                    cur.as_mut().unwrap().next =
                        Some(Box::leak(Box::new(Token::new(TokenKind::IDENT, chars, 1))));
                }
                cur = unsafe { cur.as_ref().unwrap().next.unwrap() };
                chars = &chars[1..];
                continue;
            }
            _ => {}
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
pub fn is_ident_v1(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => {return true;},
        _ => return false,
    }
}

#[allow(dead_code)]
pub fn is_ident_v2(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9'=> return true,
        _ => return false,
    }
}

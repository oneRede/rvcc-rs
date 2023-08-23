use std::{slice, str};

use crate::{
    rvcc::{get_token_ref, Token, TokenKind, TokenWrap},
    utils::{error_at, error_token, get_num_from_chars, read_punct},
};

pub static mut CURRENT_INPUT: Option<&[char]> = None;
pub static mut CURRENT_STR: Option<&str> = None;

#[allow(dead_code)]
pub fn equal(token: &Token, s: &str) -> bool {
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
pub fn skip<'a>(token: &Token, s: &str) -> Option<*mut Token> {
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
pub fn tokenize(mut chars: &'static [char]) -> TokenWrap {
    let mut head: TokenWrap = TokenWrap::empty();
    let mut cur = head.clone();

    loop {
        if chars.len() == 0 {
            cur.set_next(Box::leak(Box::new(Token::new(TokenKind::EOF, chars, 0))));

            head.reset_by_next();
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
            cur.set_next(Box::leak(Box::new(Token::new(
                TokenKind::Num,
                chars,
                num.to_string().len(),
            ))));

            chars = cs;
            cur.reset_by_next();
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

            cur.set_next(Box::leak(Box::new(Token::new(
                TokenKind::IDENT,
                chars,
                len_ident,
            ))));
            cur.reset_by_next();
            chars = &chars[len_ident..];
            continue;
        }

        match chars[0] {
            'a'..='z' => {
                cur.set_next(Box::leak(Box::new(Token::new(TokenKind::IDENT, chars, 1))));
                cur.set(cur.next());
                chars = &chars[1..];
                continue;
            }
            _ => {}
        }

        let len_punct = read_punct(chars);
        if len_punct > 0 {
            cur.set_next(Box::leak(Box::new(Token::new(
                TokenKind::Punct,
                chars,
                len_punct,
            ))));
            cur.reset_by_next();
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
        if is_keyword(get_token_ref(tk)) {
            unsafe { tk.as_mut().unwrap().kind = TokenKind::KEYWORD }
        }
    }
}

#[allow(dead_code)]
fn is_keyword(token: &Token) -> bool {
    let keywords = ["return", "if", "else", "for", "while", "int"];

    for kw in keywords {
        if equal(token, kw) {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
pub fn consume(mut token: TokenWrap, s: &str) -> (bool, TokenWrap) {
    if equal(token.get_ref(), s) {
        token.set(token.next());
        return (true, token);
    }
    return (false, token);
}

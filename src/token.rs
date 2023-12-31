use std::{
    fs::File,
    io::{self, Read},
    str, vec,
};

use self::TokenKind::*;
use crate::{
    ty::{TyWrap, TypeKind},
    utils::{add_line_numbers, error_at, error_token, read_punct},
};

pub static mut CURRENT_INPUT: Option<&[char]> = None;
pub static mut CURRENT_FILENAEM: Option<&'static str> = None;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    IDENT,
    Punct,
    NUM,
    EOF,
    KEYWORD,
    STR,
}

impl ToString for TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Punct => "Punct".to_string(),
            TokenKind::IDENT => "IDENT".to_string(),
            TokenKind::NUM => "NUM".to_string(),
            TokenKind::EOF => "EOF".to_string(),
            TokenKind::KEYWORD => "KEYWORD".to_string(),
            TokenKind::STR => "STR".to_string(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub next: TokenWrap,
    pub val: i64,
    pub loc: Option<&'static [char]>,
    pub len: usize,
    pub ty: TyWrap,
    pub stri: Vec<usize>,
    pub line_no: usize,
}

#[allow(dead_code)]
impl Token{
    pub fn new()-> Self{
        Self {
            kind: TokenKind::NUM,
            next: TokenWrap::empty(),
            val: 0,
            loc: None,
            len: 0,
            ty: TyWrap::empty(),
            stri: vec![],
            line_no: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct TokenWrap {
    pub ptr: Option<*mut Token>,
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
impl TokenWrap {
    pub fn new(kind: TokenKind, loc: &'static [char], len: usize) -> Self {
        let token = Token::new();
        let token: Option<*mut Token> = Some(Box::leak(Box::new(token)));
        
        let token = Self { ptr: token };
        token.set_kind(kind);
        token.set_loc(loc);
        token.set_len(len);
        token
    }

    pub fn init() -> Self {
        let token = Token::new();
        let tk: Option<*mut Token> = Some(Box::leak(Box::new(token)));
        let token = Self { ptr: tk };
        token
    }

    pub fn empty() -> Self {
        Self { ptr: None }
    }

    pub fn pointer(&self) -> *mut *mut Token{
        return Box::leak(Box::new(self.ptr.unwrap()))
    }

    pub fn set_next(self, next: TokenWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next };
    }

    pub fn set_kind(&self, kind: TokenKind) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().kind = kind };
    }

    pub fn set_val(self, val: i64) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().val = val };
    }

    pub fn set_len(self, len: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().len = len };
    }

    pub fn set_ty(self, ty: TyWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().ty = ty };
    }

    pub fn set_stri(self, stri: Vec<usize>) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().stri = stri };
    }

    pub fn set_nxt(self, next: TokenWrap) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().next = next };
    }

    pub fn set_loc(self, loc: &'static [char]) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().loc = Some(loc)};
    }

    pub fn nxt(&self) -> TokenWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().next }
    }

    pub fn kind(&self) -> TokenKind {
        unsafe { self.ptr.unwrap().as_ref().unwrap().kind }
    }

    pub fn len(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().len }
    }

    pub fn val(&self) -> i64 {
        unsafe { self.ptr.unwrap().as_ref().unwrap().val }
    }

    pub fn loc(&self) -> Option<&[char]> {
        unsafe { self.ptr.unwrap().as_ref().unwrap().loc }
    }

    pub fn line_no(&self) -> usize {
        unsafe { self.ptr.unwrap().as_ref().unwrap().line_no }
    }

    pub fn set_line_no(self, line_no: usize) {
        unsafe { self.ptr.unwrap().as_mut().unwrap().line_no = line_no };
    }

    pub fn stri(&self) -> Vec<usize> {
        let mut v = vec![];
        for c in unsafe { &self.ptr.unwrap().as_ref().unwrap().stri } {
            v.push(*c);
        }
        v
    }

    pub fn ty(&self) -> TyWrap {
        unsafe { self.ptr.unwrap().as_ref().unwrap().ty }
    }
}

impl ToString for TokenWrap {
    fn to_string(&self) -> String {
        if self.ptr.is_none() {
            return "None".to_string();
        }

        let loc: String = self.loc().unwrap()[..self.len()].iter().collect();

        let s = "{".to_string()
            + "\"kind\":"
            + "\""
            + &self.kind().to_string()
            + "\","
            + "\"val\":"
            + "\""
            + &self.val().to_string()
            + "\","
            + "\"loc\":"
            + "\""
            + &loc
            + "\","
            + "\"len\":"
            + "\""
            + &self.len().to_string()
            + "\""
            + "}";
        return s;
    }
}

#[allow(dead_code)]
pub fn equal(token: TokenWrap, s: &str) -> bool {
    let mut loc = "".to_string();
    for c in &token.loc().unwrap()[..token.len()] {
        loc += &c.to_string();
    }
    if &loc == s {
        return true;
    } else {
        return false;
    }
}

#[allow(dead_code)]
pub fn str_to_chars(s: &str) -> &[char] {
    Box::leak(Box::new(
        s.chars().into_iter().collect::<Vec<char>>(),
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
pub fn get_num(token: TokenWrap) -> i64 {
    if token.kind() != NUM {
        error_token(token, "expect a num");
    }
    token.val()
}

#[allow(dead_code)]
pub fn tokenize(file_name: &'static str, mut chars: &'static [char]) -> TokenWrap {
    unsafe { CURRENT_FILENAEM = Some(file_name) };
    let mut head: TokenWrap = TokenWrap::init();
    let mut cur = head;

    loop {
        if chars.len() == 0 {
            cur.set_next(TokenWrap::new(EOF, chars, 0));

            head = head.nxt();
            add_line_numbers(head.nxt());
            convert_keyword(head);
            return head;
        }

        if chars[0] == '/' && chars[1] == '/' {
            chars = &chars[2..];
            while chars[0] != '\n' {
                chars = &chars[1..];
            }
            continue;
        }

        if chars[0] == '/' && chars[1] == '*' {
            chars = &chars[2..];
            loop {
                if chars[0] == '*' && chars[1] == '/' {
                    chars = &chars[2..];
                    break;
                }
                if chars.len() == 0 {
                    error_at(chars.as_ptr(), "unclosed block comment");
                }
                chars = &chars[1..];
            }
            continue;
        }

        let c: char = chars[0];
        if c.is_whitespace() {
            chars = &chars[1..];
            continue;
        }

        if chars[0].is_ascii_digit() {
            let (tk, chs) = read_int_literal(chars);
            cur.set_next(tk);
            cur = cur.nxt();
            chars = chs;
            continue;
        }

        if chars[0] == '\"' {
            cur.set_nxt(read_string_literal(chars));
            cur = cur.nxt();
            chars = &chars[cur.len()..];
            continue;
        }

        if chars[0] == '\'' {
            let (tk, chs) = read_char_literal(chars);
            cur.set_nxt(tk);
            cur = cur.nxt();
            chars = chs;
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
        }
    }
}

#[allow(dead_code)]
fn is_keyword(token: TokenWrap) -> bool {
    let keywords = [
        "return", "if", "else", "for", "while", "int", "sizeof", "char", "struct", "union", "long",
        "short", "void", "typedef", "_Bool", "enum", "static", "goto", "break", "continue",
        "switch", "case", "default",
    ];

    for kw in keywords {
        if equal(token, kw) {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
pub fn consume(token: &mut TokenWrap, s: &str) -> bool {
    if equal(*token, s) {
        token.ptr = token.nxt().ptr;
        return true;
    }
    return false;
}

#[allow(dead_code)]
pub fn read_string_literal(chars: &'static [char]) -> TokenWrap {
    let mut start = chars;
    start = &start[1..];
    let mut buf: Vec<usize> = vec![];
    while start[0] != '\"' {
        if start[0] == '\\' {
            start = &start[1..];
            let (c, cs) = read_escaped_char(start);
            let l = start.len() - cs.len();
            buf.push(c);
            start = &start[l..];
        } else {
            buf.push(*start.get(0).unwrap() as usize);
            start = &start[1..];
        }
    }
    buf.push(0);
    let n_chars = chars.len() - start.len();

    let token = TokenWrap::new(TokenKind::STR, chars, n_chars + 1);
    let ty = TyWrap::new_array_ty(TyWrap::new_with_kind(TypeKind::CHAR), n_chars as i64);
    token.set_ty(ty);
    token.set_stri(buf);

    return token;
}

#[allow(dead_code)]
pub fn read_escaped_char(mut chars: &'static [char]) -> (usize, &[char]) {
    if chars[0] >= '0' && chars[0] <= '7' {
        let mut c = chars[0] as usize - '0' as usize;
        if chars[1] >= '0' && chars[1] <= '7' {
            c = (c << 3) + chars[1] as usize - '0' as usize;
            if chars[2] >= '0' && chars[2] <= '7' {
                c = (c << 3) + chars[2] as usize - '0' as usize;
                return (c, &chars[3..]);
            }
            return (c, &chars[2..]);
        }
        return (c, &chars[1..]);
    }

    if chars[0] == 'x' {
        chars = &chars[1..];
        if !chars[0].is_ascii_hexdigit() {
            error_at(&chars[0] as *const char, "invalid hex escape sequence");
        }

        let mut c = 0;
        while chars[0].is_ascii_hexdigit() {
            c = (c << 4) + from_hex(chars[0]);
            chars = &chars[1..];
        }
        return (c, chars);
    }

    match chars[0] {
        'a' => return ('\u{7}' as usize, &chars[1..]),
        'b' => return ('\u{8}' as usize, &chars[1..]),
        't' => return ('\u{9}' as usize, &chars[1..]),
        'n' => return ('\u{a}' as usize, &chars[1..]),
        'v' => return ('\u{b}' as usize, &chars[1..]),
        'f' => return ('\u{c}' as usize, &chars[1..]),
        'r' => return ('\u{d}' as usize, &chars[1..]),
        'e' => return ('\u{1b}' as usize, &chars[1..]),
        _ => return (chars[0] as usize, &chars[1..]),
    }
}

#[allow(dead_code)]
pub fn string_literal_end(start: &'static [char]) -> &'static [char] {
    let mut i = 0;
    for c in start {
        if *c == '\"' {
            break;
        }
        if *c == '\n' || *c == '\0' {
            error_at(c as *const char, "unclosed string literal");
        }
        i += 1;
    }
    return &start[i..];
}

#[allow(dead_code)]
pub fn from_hex(c: char) -> usize {
    if c >= '0' && c <= '9' {
        return c as usize - '0' as usize;
    }
    if c >= 'a' && c <= 'f' {
        return c as usize - 'a' as usize + 10;
    }
    return c as usize - 'A' as usize + 10;
}

#[allow(dead_code)]
pub fn read_file(path: &str) -> String {
    let mut buf = String::new();

    if path.starts_with("-") {
        for line in io::stdin().lines() {
            buf += &line.unwrap();
        }
    } else {
        let mut f = File::open(path).expect("a file path and exist file");
        let _ = f.read_to_string(&mut buf);
    }

    return buf.replace("\\n", &'\n'.to_string());
}

#[allow(dead_code)]
pub fn tokenize_file(file_path: &'static str) -> TokenWrap {
    let input_string = read_file(file_path);
    let input: &str = Box::leak(Box::new(String::from(input_string)));
    let chars: Vec<char> = input.chars().collect();
    let chars: &[char] = Box::leak(Box::new(chars));

    unsafe { CURRENT_FILENAEM = Some(input) };
    unsafe { CURRENT_INPUT = Some(chars) };

    return tokenize(file_path, chars);
}

#[allow(dead_code)]
pub fn read_char_literal(start: &'static [char]) -> (TokenWrap, &'static [char]) {
    let mut p = &start[1..];

    if p[0] == '\0' {
        error_at(&start[0] as *const char, "unclosed char literal");
    }

    let c: char;
    if p[0] == '\\' {
        let (cs, chars) = read_escaped_char(&p[1..]);
        p = chars;
        c = char::from_u32(cs as u32).unwrap();
    } else {
        c = p[0];
        p = &p[1..]
    }

    let end = str_chr(&start[1..]);

    let token = TokenWrap::new(TokenKind::NUM, start, end + 1);
    token.set_val(c as i64);
    return (token, &p[1..]);
}

#[allow(dead_code)]
pub fn str_chr(chars: &'static [char]) -> usize {
    let mut i = 0;
    for c in chars {
        if *c == '\'' {
            break;
        }
        i += 1;
    }
    return i + 1;
}

#[allow(dead_code)]
pub fn read_int_literal(start: &'static [char]) -> (TokenWrap, &'static [char]) {
    let mut p = start;
    let mut base: u32 = 10;
    if p[0] == '0' && !(p[1] == 'x' || p[1] == 'X' || p[1] == 'b' || p[1] == 'B') {
        base = 8;
    } else if (p.starts_with(&['0', 'b']) || p.starts_with(&['0', 'B']))
        && (p[2] == '0' || p[2] == '1')
    {
        p = &p[2..];
        base = 2;
    } else if (p.starts_with(&['0', 'x']) || p.starts_with(&['0', 'X'])) && p[2].is_ascii_hexdigit()
    {
        p = &p[2..];
        base = 16;
    }

    let mut str_num = "".to_string();
    for c in p {
        if c.is_ascii_digit() || (base == 16 && c.is_ascii_hexdigit()) {
            str_num += &c.to_string();
        } else {
            break;
        }
    }

    let val = i64::from_str_radix(&str_num, base).unwrap();
    let mut len = str_num.len();
    if base == 16 || base == 2 {
        len += 2;
    }
    let token = TokenWrap::new(TokenKind::NUM, start, len);
    token.set_val(val);
    return (token, &start[len..]);
}

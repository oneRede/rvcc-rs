use std::{num::ParseIntError, process::exit};

use crate::token::{TokenWrap, CURRENT_FILENAEM, CURRENT_INPUT};

#[allow(dead_code)]
pub fn get_num_from_chars(s: &[char]) -> Result<(i64, &[char]), ParseIntError> {
    let mut i: usize = 0;
    let mut num_string = "".to_string();
    for c in s {
        match c {
            '0'..='9' => {
                i += 1;
                num_string += c.to_string().as_ref();
            }
            _ => {
                break;
            }
        }
    }
    let rs: Result<i64, ParseIntError> = num_string.parse();
    match rs {
        Ok(num) => return Ok((num, &s[i..])),
        Err(e) => return Err(e),
    }
}

#[allow(dead_code)]
pub fn read_punct(ptr: &[char]) -> usize {
    if starts_with(ptr, &['=', '='])
        || starts_with(ptr, &['!', '='])
        || starts_with(ptr, &['<', '='])
        || starts_with(ptr, &['>', '='])
        || starts_with(ptr, &['-', '>'])
        || starts_with(ptr, &['+', '='])
        || starts_with(ptr, &['-', '='])
        || starts_with(ptr, &['*', '='])
        || starts_with(ptr, &['/', '='])
        || starts_with(ptr, &['+', '+'])
        || starts_with(ptr, &['-', '-'])
        || starts_with(ptr, &['%', '='])
        || starts_with(ptr, &['&', '='])
        || starts_with(ptr, &['|', '='])
        || starts_with(ptr, &['^', '='])
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
        || ptr[0] == ';'
        || ptr[0] == '{'
        || ptr[0] == '}'
        || ptr[0] == '&'
        || ptr[0] == ','
        || ptr[0] == '['
        || ptr[0] == ']'
        || ptr[0] == '\"'
        || ptr[0] == '.'
        || ptr[0] == '~'
        || ptr[0] == '%'
        || ptr[0] == '|'
        || ptr[0] == '^'
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
pub fn error_at(loc: *const char, msg: &str) {
    let mut line_no = 1;
    let mut start = unsafe { CURRENT_INPUT.unwrap().as_ptr() };
    while start.lt(&loc) {
        if unsafe { *start.as_ref().unwrap() } == '\n' {
            line_no += 1;
        }
        start = unsafe { start.add(1) };
    }

    v_error_at(line_no, loc, msg);
    exit(1);
}

#[allow(dead_code)]
pub fn error_token(token: TokenWrap, msg: &str) {
    let loc = token.loc().unwrap().as_ptr();
    v_error_at(token.line_no(), loc, msg);
    exit(1);
}

#[allow(dead_code)]
pub fn v_error_at(line_no: usize, loc: *const char, msg: &str) {
    let mut line = loc;
    let start = unsafe { CURRENT_INPUT.unwrap().as_ptr() };

    while line.gt(&start) && unsafe { *line.sub(1).as_ref().unwrap() } != '\n' {
        line = unsafe { line.sub(1) };
    }

    let mut end = loc;
    while unsafe { *end.as_ref().unwrap() } != '\n' {
        end = unsafe { end.sub(1) }
    }

    let fmt = format!("{}:{}: ", unsafe { CURRENT_FILENAEM.unwrap() }, line_no);
    println!("{}", fmt);
    let indent = fmt.len();
    println!("{}.{:?}", unsafe { end.offset_from(line) }, line);
    let pos = unsafe { loc.offset_from(line) } + indent as isize;
    eprint!("{:?}", " ".repeat(pos as usize));
    eprint!("{}", "^");
    eprintln!("{}", msg);
}

#[allow(dead_code)]
pub fn add_line_numbers(mut token: TokenWrap) {
    if token.ptr.is_none() {
        return;
    }
    let mut start = unsafe { CURRENT_INPUT.unwrap().as_ptr() };
    let mut n = 1;

    for c in unsafe { CURRENT_INPUT.unwrap() } {
        if start == token.loc().unwrap().as_ptr() {
            token.set_line_no(n);
            token = token.nxt();
        }
        if *c == '\n' {
            n += 1;
        }
        start = unsafe { start.add(1) };
    }
}

#[allow(dead_code)]
pub fn num_base_conversion(mut num: i64, src_base: i64, dst_base: i64) -> i64{
    let mut dst_base_num = 0;
    let mut i = 0;

    while num != 0{
        dst_base_num += (num % dst_base) * i64::pow(src_base, i);
        num /= dst_base;
        i +=1;
    }
    return dst_base_num
}

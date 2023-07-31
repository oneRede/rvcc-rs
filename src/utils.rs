use std::{num::ParseIntError, process::exit};

use crate::tokenize::{CURRENT_INPUT, CURRENT_STR};

#[allow(dead_code)]
pub fn get_num_from_chars(s: &[char]) -> Result<(i32, &[char]), ParseIntError> {
    let mut i: usize = 0;
    let mut num_string = "".to_string();
    for c in s {
        match c {
            '0'..='9' => {
                i += 1;
                num_string += c.to_string().as_ref();
            }
            '-' | '+' | '/' | '*' | ' ' | '(' | ')' | '=' | '<' | '>' | '!' | ';' => {
                break;
            }
            'a'..='z' =>{
                break;
            }
            _ => {
                println!("# unexcept character: {}", c);
                exit(1)
            }
        }
    }
    let rs: Result<i32, ParseIntError> = num_string.parse();
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
pub fn v_error_at(loc: *const char, msg: &str) {
    let input = unsafe { CURRENT_STR.unwrap() };
    let chars = unsafe { CURRENT_INPUT.unwrap() };
    eprintln!("{:?}", input);
    let distance = (unsafe { loc.offset_from(chars.as_ptr()) }).abs() - 1;
    eprint!("{:?}", " ".repeat(distance as usize));
    eprint!("{}", "^");
    eprintln!("{}", msg);
}

#[allow(dead_code)]
pub fn error_at(loc: *const char, msg: &str) {
    v_error_at(loc, msg);
    exit(1);
}
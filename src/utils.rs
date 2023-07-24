use std::{num::ParseIntError, process::exit};

#[allow(dead_code)]
pub fn get_str_num(s: &str) -> (isize, &str) {
    let mut i: usize = 0;
    for c in s.chars() {
        match c {
            '0'..='9' => {
                i += 1;
            }
            '-' | '+' | '/' | '*' | ' '=> {
                break;
            }
            _ => {}
        }
    }

    return ((&s[..i]).parse().unwrap(), &s[i..]);
}

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
            '-' | '+' | '/' | '*' | ' ' | '(' | ')'=> {
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
pub fn is_digit(s: &str) -> bool {
    for c in s[..1].chars() {
        match c {
            '0'..='9' => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
    false
}

#[allow(dead_code)]
fn error_at(loc: *const char, chars: &[char], input: &str, msg: &str) {
    eprintln!("{:?}", input);
    let distance = (unsafe { loc.offset_from(chars.as_ptr()) }).abs() - 1;
    eprintln!("{}", distance);
    eprint!("{:?}", " ".repeat(distance as usize));
    eprint!("{}", "^");
    eprintln!("{}", msg);
}

#[test]
fn test_get_str_num() {
    let s = "12335+67890";
    let (a, b) = get_str_num(s);
    println!("{}", a);
    println!("{}", b);
}

#[test]
fn test_is_digit() {
    let mut s = "12335+67890".to_string();
    let a = is_digit(s.as_mut());
    println!("{}", a);
    println!("{}", s);
}

#[test]
fn test_eof() {
    let s = "1234567890";
    println!("{:?}", s.as_bytes())
}

#[test]
fn test_str_p() {
    let s = "1234567890";
    let mut p = s.as_ptr();
    p = unsafe { p.add(2) };
    let s2 = std::str::from_utf8(unsafe { std::slice::from_raw_parts(p, 8) });
    println!("{}", s2.unwrap())
}

#[test]
fn test_p_u8() {
    let s = "";
    println!("{:?}", &s.as_ptr())
}

#[test]
fn test_error_at() {
    let input = "12345678";
    let chars = &['1', '2', '3', '4', '5', '6', '7', '8'];
    let loc = &chars[4..];

    // let p_chars = chars.as_ptr();
    let p_loc = loc.as_ptr();
    error_at(p_loc, chars, input, " error character")
}

#[test]
fn test_p_p() {
    let s1 = "12345".to_string();
    let s2 = "67890".to_string();

    let mut  p1: *mut String = Box::leak(Box::new(s1));
    let mut p2: *mut String = Box::leak(Box::new(s2));
    
    let mut p1_s: *mut *mut String = &mut p1;
    unsafe { *p1_s.as_mut().unwrap() = p2 };
    println!("{:?}", p1);
    println!("{:?}", p2);
    
}

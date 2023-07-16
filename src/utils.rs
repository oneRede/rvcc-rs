use std::{env::consts, mem};

#[allow(dead_code)]
pub fn get_str_num(s: &str) -> (isize, &str) {
    let mut i: usize = 0;
    for c in s.chars() {
        match c {
            '0'..='9' => {
                i += 1;
            }
            '-' | '+' => {
                break;
            }
            _ => {}
        }
    }

    return ((&s[..i]).parse().unwrap(), &s[i..]);
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
fn test_eof(){
    let s = "1234567890";
    println!("{:?}", s.as_bytes())
}

#[test]
fn test_str_p(){
    let s = "1234567890";
    let mut p = s.as_ptr();
    p = unsafe { p.add(2) };
    let s2 = std::str::from_utf8(unsafe { std::slice::from_raw_parts(p, 8) });
    println!("{}", s2.unwrap())
}

#[test]
fn test_p_u8(){
    let s = "";
    println!("{:?}", &s.as_ptr())
}

#[test]
fn test_p_str(){
    let s = "1234567890".to_string();
    let p: *const String = Box::leak(Box::new(s));
    println!("{:?}", p);
    println!("{}", mem::size_of::<*const String>());
    println!("{:?}", unsafe{p.add(2)});
    println!("{:?}", unsafe{(p as *mut u8).add(1)});
    println!("{}", mem::size_of::<*const u8>());
    println!("{}", mem::size_of::<Box<u8>>());
    println!("{}", mem::size_of::<Vec<String>>());
    println!("{}", mem::size_of::<Vec<u8>>())
}

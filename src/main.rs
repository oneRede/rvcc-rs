use std::{env, process::exit};

mod utils;
mod codegen;
mod parse;

use parse::{CURRENT_STR, CURRENT_INPUT, tokenize, Token, expr, TokenKind, error_token};
use codegen::codegen;

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

    if unsafe { token.as_ref().unwrap().kind } != TokenKind::Eof {
        error_token(unsafe { token.as_ref().unwrap() }, "extra token");
    }

    codegen(node);
    return;
}

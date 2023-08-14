use std::{env, process::exit};

mod codegen;
mod parse;
mod rvcc;
mod tokenize;
mod utils;
mod ty;

use crate::parse::parse;
use codegen::codegen;
use tokenize::{tokenize, CURRENT_INPUT, CURRENT_STR};

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

    let token = tokenize(chars);
    let prog = parse(token);

    codegen(prog);
    return;
}

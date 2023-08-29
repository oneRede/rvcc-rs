use std::{env, process::exit};

mod codegen;
mod node;
mod obj;
mod parse;
mod token;
mod ty;
mod utils;

use codegen::codegen;
use parse::parse;
use token::{tokenize, CURRENT_INPUT, CURRENT_STR};

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
    println!("#token {}",token.to_string());
    let prog = parse(token);
    println!("#prog {}", prog.body().to_string());

    codegen(prog);
    return;
}

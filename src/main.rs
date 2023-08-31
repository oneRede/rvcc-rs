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
use token::tokenize_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if &args.len() != &2 {
        println!("{}: invalid number of arguments\n", &args.get(0).unwrap());
        exit(1)
    }

    let input: &str = Box::leak(Box::new(String::from(&args[1])));

    let token = tokenize_file(input);
    let prog = parse(token);

    codegen(prog);
    return;
}

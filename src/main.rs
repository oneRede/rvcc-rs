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

pub static mut OPT_O: Option<String> = None;
pub static mut INPUT_PATH: Option<String> = None;

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

#[allow(dead_code)]
pub fn usage(status: i32) {
    println!("rvcc [ -o <path> ] <file>\n");
    exit(status);
}

#[allow(dead_code)]
pub fn parse_args(argc: usize, argv: Vec<String>) {
    let mut i = 1;
    while i< argc {
        if argv.get(i).unwrap().starts_with("--help") {
            usage(0);
        }

        if argv.get(i).unwrap() == "-o" {
            i += 1;
            if argv[i].len() == 0 {
                usage(1);
            }
            unsafe { OPT_O = argv.get(i).cloned() };
            continue;
        }

        if argv.get(i).unwrap().starts_with("-o") {
            i += 1;
            if argv[i].len() == 0 {
                usage(1);
            }
            unsafe { OPT_O = Some(String::from(&argv.get(i).unwrap()[2..])) };
            continue;
        }
        let mut chars = argv[i].chars();

        if chars.next().unwrap() == '-' && chars.next().unwrap() != '\u{0}' {
            panic!("unknown argument: {}", argv[i]);
        }

        unsafe { INPUT_PATH = Some(String::from(argv.get(i).unwrap())) };

    }
    if unsafe { INPUT_PATH.is_none() } {
        panic!("no input files");
    }
}

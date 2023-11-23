use std::{env, fs::File, process::exit};

mod codegen;
mod node;
mod obj;
mod parse;
mod scope;
mod token;
mod ty;
mod utils;
mod test;

use codegen::codegen;
use parse::parse;
use token::tokenize_file;

pub static mut OPT_O: Option<String> = None;
pub static mut INPUT_PATH: Option<String> = None;

#[allow(dead_code)]
pub fn usage(status: i32) {
    println!("rvcc [ -o <path> ] <file>\n");
    exit(status);
}

#[allow(dead_code)]
pub fn parse_args(argc: usize, argv: Vec<String>) {
    let mut i = 1;
    while i < argc {
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
            if argv[i].len() == 0 {
                usage(1);
            }
            unsafe { OPT_O = Some(String::from(&argv.get(i).unwrap()[2..])) };
            i += 1;
            continue;
        }
        let mut chars = argv[i].chars();

        if chars.next().unwrap() == '-' && argv[i].len() > 1 {
            panic!("unknown argument: {}", argv[i]);
        }

        unsafe { INPUT_PATH = Some(String::from(argv.get(i).unwrap())) };
        i += 1;
    }
    if unsafe { INPUT_PATH.is_none() } {
        panic!("no input files");
    }
}

#[allow(dead_code)]
pub fn open_file(file_path: &str) -> Result<File, std::io::Error> {
    File::create(file_path)
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    let argc = argv.len();
    parse_args(argc, argv);

    let input: &str = unsafe { INPUT_PATH.as_ref().unwrap() };

    let token = tokenize_file(input);
    let prog = parse(token);

    let out = open_file(unsafe { OPT_O.as_ref().unwrap() }).unwrap();
    codegen(prog, out);
    return;
}

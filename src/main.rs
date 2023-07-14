use std::{env, process::exit};

mod utils;

use utils::get_str_num;
fn main() {
    let args: Vec<String> = env::args().collect();
    if &args.len() != &2 {
        println!("{}: invalid number of arguments\n", &args.get(0).unwrap());
        exit(1)
    }
    let mut p: &str = &args[1];

    println!("  .globl main");
    println!("main:");
    let (n, s) = get_str_num(p);
    p = s;
    println!("  li a0, {}", n);

    loop {
        if p.len() == 0{
            break;
        }
        if p.starts_with("+"){
            let (n, s) = get_str_num(&p[1..]);
            println!("  addi a0, a0, {}", n);
            p = s;

        } else if p.starts_with("-") {
            let (n, s) = get_str_num(&p[1..]);
            println!("  addi a0, a0, -{}", n);
            p = s;
        } else {
            println!("unexcept character: {}", &p[..1]);
            exit(1)
        }
    }

    println!("  ret");
    exit(0)
}
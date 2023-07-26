use crate::{parse::Node, parse::NodeKind};

pub static mut DEPTH: usize = 0;

#[allow(dead_code)]
pub fn push() {
    println!("  addi sp, sp, -8");
    println!("  sd a0, 0(sp)");
    unsafe { DEPTH += 1 };
}

#[allow(dead_code)]
pub fn pop(reg: &str) {
    println!("  ld {}, 0(sp)", reg);
    println!("  addi sp, sp, 8");
    unsafe { DEPTH -= 1 };
}

#[allow(dead_code)]
pub fn gen_expr(node: *mut Node) {
    match unsafe { node.as_ref().unwrap().kind } {
        NodeKind::Num => {
            println!("  li a0, {:?}", unsafe { node.as_ref().unwrap().val });
            return;
        }
        NodeKind::NEG => {
            gen_expr(unsafe { node.as_ref().unwrap().lhs }.unwrap());
            println!("  neg a0, a0");
            return;
        }
        _ => {}
    }

    gen_expr(unsafe { node.as_ref().unwrap().rhs }.unwrap());
    push();
    gen_expr(unsafe { node.as_ref().unwrap().lhs }.unwrap());
    pop("a1");

    match unsafe { node.as_ref().unwrap().kind } {
        NodeKind::Add => {
            println!("  add a0, a0, a1");
            return;
        }
        NodeKind::Sub => {
            println!("  sub a0, a0, a1");
            return;
        }
        NodeKind::Mul => {
            println!("  mul a0, a0, a1");
            return;
        }
        NodeKind::Div => {
            println!("  div a0, a0, a1");
            return;
        }
        NodeKind::EQ => {
            println!("  xor a0, a0, a1");
            println!("  seqz a0, a0");
            return;
        }
        NodeKind::NE => {
            println!("  xor a0, a0, a1");
            println!("  snez a0, a0");
            return;
        }
        NodeKind::LT => {
            println!("  slt a0, a0, a1");
            return;
        }
        NodeKind::LE => {
            println!("  slt a0, a1, a0");
            println!("  xori a0, a0, 1");
            return;
        }
        _ => {
            return;
        }
    }
}

#[allow(dead_code)]
pub fn codegen(node: *mut Node){
    println!("  .globl main");
    println!("main:");
    gen_expr(node);
    println!("  ret");
    assert!(unsafe { DEPTH == 0 });
}


use crate::rvcc::{
    get_fuction_locals, get_fuction_stack_size, get_node_body, get_node_kind, get_node_lhs,
    get_node_next, get_node_rhs, get_node_val, get_node_var, get_obj_offset,
    set_fuction_stack_size, set_obj_offset, Function, Node, NodeKind, ObjIter, get_fuction_body,
};

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
pub fn align_to(n: i64, align: i64) -> i64 {
    return (n + align - 1) / align * align;
}

#[allow(dead_code)]
pub fn gen_addr(node: *mut Node) {
    if get_node_kind(node) == NodeKind::VAR {
        let offset = get_obj_offset(get_node_var(node));
        println!("  addi a0, fp, {}", offset);
        return;
    }
    println!("not an value");
}

#[allow(dead_code)]
pub fn gen_expr(node: *mut Node) {
    match get_node_kind(node) {
        NodeKind::Num => {
            println!("  li a0, {:?}", get_node_val(node));
            return;
        }
        NodeKind::NEG => {
            gen_expr(get_node_lhs(node));
            println!("  neg a0, a0");
            return;
        }
        NodeKind::VAR => {
            gen_addr(node);
            println!("  ld a0, 0(a0)");
            return;
        }
        NodeKind::ASSIGN => {
            gen_addr(get_node_lhs(node));
            push();
            gen_expr(get_node_rhs(node));
            pop("a1");
            println!("  sd a0, 0(a1)");
            return;
        }
        _ => {}
    }

    gen_expr(get_node_rhs(node));
    push();
    gen_expr(get_node_lhs(node));
    pop("a1");

    match get_node_kind(node) {
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
fn gen_stmt(mut node: *mut Node) {
    match get_node_kind(node) {
        NodeKind::BLOCK => {
            if get_node_body(node).is_none() {
                return;
            }
            node = get_node_body(node).unwrap();
            loop {
                gen_stmt(node);
                if get_node_next(node).is_none() {
                    return;
                }
                node = get_node_next(node).unwrap();
            }
        }

        NodeKind::RETURN => {
            gen_expr(get_node_lhs(node));
            println!("  j .L.return");
            return;
        }
        NodeKind::ExprStmt => {
            gen_expr(get_node_lhs(node));
            return;
        }
        _ => {}
    }
    println!("invalid statement");
}

#[allow(dead_code)]
pub fn assign_l_var_offsets(prog: *mut Function) {
    let mut offset = 0;
    let var = ObjIter::new(get_fuction_locals(prog));

    for obj in var {
        offset += 8;
        set_obj_offset(obj, -offset);
    }
    set_fuction_stack_size(prog, align_to(offset, 16));
}

#[allow(dead_code)]
pub fn codegen(prog: *mut Function) {
    assign_l_var_offsets(prog);
    println!("  .globl main");
    println!("main:");

    println!("  addi sp, sp, -8");
    println!("  sd fp, 0(sp)");
    println!("  mv fp, sp");
    println!("  addi sp, sp, -{}", get_fuction_stack_size(prog));

    let node = get_fuction_body(prog).unwrap();

    gen_stmt(node);
    assert!(unsafe { DEPTH == 0 });

    println!(".L.return:");
    println!("  mv sp, fp");
    println!("  ld fp, 0(sp)");
    println!("  addi sp, sp, 8");

    println!("  ret");
}

use crate::{rvcc::{
    get_fuction_body, get_fuction_locals, get_fuction_stack_size, get_node_body, get_node_cond,
    get_node_els, get_node_inc, get_node_init, get_node_kind, get_node_lhs, get_node_next,
    get_node_rhs, get_node_then, get_node_val, get_node_var, get_obj_name, get_obj_offset,
    set_fuction_stack_size, set_obj_offset, Function, Node, NodeKind, ObjIter, get_node_token,
}, utils::error_token};

pub static mut DEPTH: usize = 0;
pub static mut I: i64 = 1;

#[allow(dead_code)]
pub fn count() -> i64 {
    unsafe {
        I += 1;
        return I;
    }
}

#[allow(dead_code)]
pub fn push() {
    println!("  # 压栈，将a0的值存入栈顶");
    println!("  addi sp, sp, -8");
    println!("  sd a0, 0(sp)");
    unsafe { DEPTH += 1 };
}

#[allow(dead_code)]
pub fn pop(reg: &str) {
    println!("  # 弹栈，将栈顶的值存入{}", reg);
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
        println!(
            "  # 获取变量{}的栈内地址为{}(fp)",
            get_obj_name(get_node_var(node)),
            get_obj_offset(get_node_var(node))
        );
        println!("  addi a0, fp, {}", offset);
        return;
    }
    error_token(get_node_token(node).get_ref(), "not an lvalue");
}

#[allow(dead_code)]
pub fn gen_expr(node: *mut Node) {
    match get_node_kind(node) {
        NodeKind::Num => {
            println!("  # 将{}加载到a0中", get_node_val(node));
            println!("  li a0, {:?}", get_node_val(node));
            return;
        }
        NodeKind::NEG => {
            gen_expr(get_node_lhs(node));
            println!("  # 对a0值进行取反");
            println!("  neg a0, a0");
            return;
        }
        NodeKind::VAR => {
            gen_addr(node);
            println!("  # 读取a0中存放的地址，得到的值存入a0");
            println!("  ld a0, 0(a0)");
            return;
        }
        NodeKind::ASSIGN => {
            gen_addr(get_node_lhs(node));
            push();
            gen_expr(get_node_rhs(node));
            pop("a1");
            println!("  # 将a0的值，写入到a1中存放的地址");
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
            println!("  # a0+a1，结果写入a0");
            println!("  add a0, a0, a1");
            return;
        }
        NodeKind::Sub => {
            println!("  # a0-a1，结果写入a0");
            println!("  sub a0, a0, a1");
            return;
        }
        NodeKind::Mul => {
            println!("  # a0×a1，结果写入a0");
            println!("  mul a0, a0, a1");
            return;
        }
        NodeKind::Div => {
            println!("  # a0÷a1，结果写入a0");
            println!("  div a0, a0, a1");
            return;
        }
        NodeKind::EQ => {
            println!("  xor a0, a0, a1");
            println!("  seqz a0, a0");
            return;
        }
        NodeKind::NE => {
            let p = if get_node_kind(node) == NodeKind::EQ {
                "="
            } else {
                "!="
            };
            println!("  # 判断是否a0{}a1", p);
            println!("  xor a0, a0, a1");
            println!("  snez a0, a0");
            return;
        }
        NodeKind::LT => {
            println!("  # 判断a0<a1");
            println!("  slt a0, a0, a1");
            return;
        }
        NodeKind::LE => {
            println!("  # 判断是否a0≤a1");
            println!("  slt a0, a1, a0");
            println!("  xori a0, a0, 1");
            return;
        }
        _ => {
        }
    }
    error_token(get_node_token(node).get_ref(), "invalid expression");
}

#[allow(dead_code)]
fn gen_stmt(mut node: *mut Node) {
    match get_node_kind(node) {
        NodeKind::IF => {
            let c = count();
            println!("\n# =====分支语句{}==============", c);
            println!("\n# Cond表达式{}", c);
            gen_expr(get_node_cond(node).unwrap());
            println!("  # 若a0为0，则跳转到分支{}的.L.else.{}段", c, c);
            println!("  beqz a0, .L.else.{}", c);

            println!("\n# Then语句{}", c);
            gen_stmt(get_node_then(node).unwrap());
            println!("  # 跳转到分支{}的.L.end.{}段\n", c, c);
            println!("  j .L.end.{}", c);
            println!("\n# Else语句{}", c);
            println!("# 分支{}的.L.else.{}段标签", c, c);
            println!(".L.else.{}:", c);

            if !get_node_els(node).is_none() {
                gen_stmt(get_node_els(node).unwrap())
            }
            println!("\n# 分支{}的.L.end.{}段标签", c, c);
            println!(".L.end.{}:", c);
            return;
        }

        NodeKind::FOR => {
            let c = count();
            println!("\n# =====循环语句{}===============", c);
            if !get_node_init(node).is_none() {
                println!("\n# Init语句%{}", c);
                gen_stmt(get_node_init(node).unwrap());
            }

            println!("\n# 循环{}的.L.begin.{}段标签", c, c);
            println!(".L.begin.{}:", c);

            println!("# Cond表达式{}", c);
            if !get_node_cond(node).is_none() {
                gen_expr(get_node_cond(node).unwrap());
                println!("  # 若a0为0，则跳转到循环{}的.L.end.{}段", c, c);
                println!("  beqz a0, .L.end.{}", c);
            }

            println!("\n# Then语句{}", c);
            gen_stmt(get_node_then(node).unwrap());

            if !get_node_inc(node).is_none() {
                println!("\n# Inc语句{}", c);
                gen_expr(get_node_inc(node).unwrap())
            }

            println!("  # 跳转到循环{}的.L.begin.{}段", c, c);
            println!("  j .L.begin.{}", c);
            println!("\n# 循环{}的.L.end.{}段标签", c, c);
            println!(".L.end.{}:", c);
            return;
        }

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
            println!("# 返回语句");
            gen_expr(get_node_lhs(node));
            println!("  # 跳转到.L.return段");
            println!("  j .L.return");
            return;
        }
        NodeKind::ExprStmt => {
            gen_expr(get_node_lhs(node));
            return;
        }
        _ => {}
    }
    error_token(get_node_token(node).get_ref(), "invalid statement");
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
    println!("  # 定义全局main段");
    println!("  .globl main");
    println!("\n# =====程序开始===============");
    println!("# main段标签，也是程序入口段");
    println!("main:");

    println!("  # 将fp压栈，fp属于“被调用者保存”的寄存器，需要恢复原值");
    println!("  addi sp, sp, -8");
    println!("  sd fp, 0(sp)");
    println!("  # 将sp的值写入fp");
    println!("  mv fp, sp");

    println!("  # sp腾出StackSize大小的栈空间");
    println!("  addi sp, sp, -{}", get_fuction_stack_size(prog));

    let node = get_fuction_body(prog).unwrap();

    println!("\n# =====程序主体===============");
    gen_stmt(node);
    assert!(unsafe { DEPTH == 0 });

    println!("\n# =====程序结束===============");
    println!("# return段标签");
    println!(".L.return:");
    println!("  # 将fp的值写回sp");
    println!("  mv sp, fp");
    println!("  # 将最早fp保存的值弹栈，恢复fp和sp");
    println!("  ld fp, 0(sp)");
    println!("  addi sp, sp, 8");

    println!("  # 返回a0值给系统调用");
    println!("  ret");
}

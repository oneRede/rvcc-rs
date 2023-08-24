use crate::{
    rvcc::{
        NodeKind,
        NodeWrap, TyWrap, TypeKind, FunctionWrap,
    },
    utils::error_token,
};

pub static mut DEPTH: usize = 0;
pub static mut I: i64 = 1;

pub static ARG_REG: [&str; 6] = ["a0", "a1", "a2", "a3", "a4", "a5"];
pub static mut FUNCTION: FunctionWrap = FunctionWrap::empty();

#[allow(dead_code)]
pub fn count() -> i64 {
    unsafe {
        I += 1;
        return I;
    }
}

#[allow(dead_code)]
pub fn push() {
    println!("  # 压栈,将a0的值存入栈顶");
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
pub fn gen_addr(node: NodeWrap) {
    match node.kind() {
        NodeKind::VAR => {
            let offset = node.var().offset();
            println!(
                "  # 获取变量{}的栈内地址为{}(fp)",
                node.var().name(),
                node.var().offset()
            );
            println!("  addi a0, fp, {}", offset);
            return;
        }
        NodeKind::DEREF => {
            gen_expr(node.lhs());
            return;
        }
        _ => {}
    }
    error_token(node.token(), "not an lvalue");
}

#[allow(dead_code)]
pub fn gen_expr(node: NodeWrap) {
    match node.kind() {
        NodeKind::Num => {
            println!("  # 将{}加载到a0中", node.val());
            println!("  li a0, {}", node.val());
            return;
        }
        NodeKind::NEG => {
            gen_expr(node.lhs());
            println!("  # 对a0值进行取反");
            println!("  neg a0, a0");
            return;
        }
        NodeKind::VAR => {
            gen_addr(node);
            load(node.ty());
            return;
        }
        NodeKind::DEREF => {
            gen_expr(node.lhs());
            load(node.ty());
            return;
        }
        NodeKind::ADDR => {
            gen_addr(node.lhs());
            return;
        }
        NodeKind::ASSIGN => {
            gen_addr(node.lhs());
            push();
            gen_expr(node.rhs());
            store();
            return;
        }
        NodeKind::FUNCALL => {
            let mut n_args = 0;

            let mut arg = node.args();
            while !arg.ptr.is_none() {
                gen_expr(arg);
                push();
                arg = arg.next();
                n_args += 1;
            }

            let mut index: isize = n_args - 1;
            while index >= 0 {
                pop(ARG_REG[index as usize]);
                index -= 1;
            }

            println!("\n  # 调用函数{}", node.func_name());
            println!("  call {}", node.func_name());
            return;
        }
        _ => {}
    }

    gen_expr(node.rhs());
    push();
    gen_expr(node.lhs());
    pop("a1");

    match node.kind() {
        NodeKind::Add => {
            println!("  # a0+a1,结果写入a0");
            println!("  add a0, a0, a1");
            return;
        }
        NodeKind::Sub => {
            println!("  # a0-a1,结果写入a0");
            println!("  sub a0, a0, a1");
            return;
        }
        NodeKind::Mul => {
            println!("  # a0*a1,结果写入a0");
            println!("  mul a0, a0, a1");
            return;
        }
        NodeKind::Div => {
            println!("  # a0÷a1,结果写入a0");
            println!("  div a0, a0, a1");
            return;
        }
        NodeKind::EQ => {
            println!("  xor a0, a0, a1");
            println!("  seqz a0, a0");
            return;
        }
        NodeKind::NE => {
            let p = if node.kind() == NodeKind::EQ {
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
        _ => {}
    }
    error_token(node.token(), "invalid expression");
}

#[allow(dead_code)]
fn gen_stmt(mut node: NodeWrap) {
    match node.kind() {
        NodeKind::IF => {
            let c = count();
            println!("\n# =====分支语句{}==============", c);
            println!("\n# Cond表达式{}", c);
            gen_expr(node.cond());
            println!("  # 若a0为0,则跳转到分支{}的.L.else.{}段", c, c);
            println!("  beqz a0, .L.else.{}", c);

            println!("\n# Then语句{}", c);
            gen_stmt(node.then());
            println!("  # 跳转到分支{}的.L.end.{}段\n", c, c);
            println!("  j .L.end.{}", c);
            println!("\n# Else语句{}", c);
            println!("# 分支{}的.L.else.{}段标签", c, c);
            println!(".L.else.{}:", c);

            if !node.els().ptr.is_none() {
                gen_stmt(node.els())
            }
            println!("\n# 分支{}的.L.end.{}段标签", c, c);
            println!(".L.end.{}:", c);
            return;
        }

        NodeKind::FOR => {
            let c = count();
            println!("\n# =====循环语句{}===============", c);
            if !node.init().ptr.is_none() {
                println!("\n# Init语句%{}", c);
                gen_stmt(node.init());
            }

            println!("\n# 循环{}的.L.begin.{}段标签", c, c);
            println!(".L.begin.{}:", c);

            println!("# Cond表达式{}", c);
            if !node.cond().ptr.is_none() {
                gen_expr(node.cond());
                println!("  # 若a0为0,则跳转到循环{}的.L.end.{}段", c, c);
                println!("  beqz a0, .L.end.{}", c);
            }

            println!("\n# Then语句{}", c);
            gen_stmt(node.then());

            if !node.inc().ptr.is_none() {
                println!("\n# Inc语句{}", c);
                gen_expr(node.inc())
            }

            println!("  # 跳转到循环{}的.L.begin.{}段", c, c);
            println!("  j .L.begin.{}", c);
            println!("\n# 循环{}的.L.end.{}段标签", c, c);
            println!(".L.end.{}:", c);
            return;
        }

        NodeKind::BLOCK => {
            if node.body().ptr.is_none() {
                return;
            }
            node = node.body();
            loop {
                gen_stmt(node);
                if node.next().ptr.is_none() {
                    return;
                }
                node = node.next()
            }
        }

        NodeKind::RETURN => {
            println!("# 返回语句");
            gen_expr(node.lhs());
            println!(
                "  # 跳转到.L.return.{}段",
                unsafe { FUNCTION }.name()
            );
            println!("  j .L.return.{}", unsafe { FUNCTION }.name());
            return;
        }
        NodeKind::ExprStmt => {
            gen_expr(node.lhs());
            return;
        }
        _ => {}
    }
    error_token(node.token(), "invalid statement");
}

#[allow(dead_code)]
pub fn assign_l_var_offsets(prog: FunctionWrap) {
    let mut func = prog;
    while !func.ptr.is_none() {
        let mut offset = 0;
        let var = func.locals();
        for obj in var {
            offset += obj.ty().size() as i64;
            obj.set_offset(-offset);
        }
        func.set_stack_size(align_to(offset, 16));
        func = func.nxt();
    }
}

#[allow(dead_code)]
pub fn codegen(prog: FunctionWrap) {
    assign_l_var_offsets(prog);
    let mut func = prog;
    while !func.ptr.is_none() {
        println!("\n  # 定义全局{}段", func.name());
        println!("  .globl {}", func.name());
        println!("# ====={}段开始===============", func.name());
        println!("# {}段标签", func.name());
        println!("{}:", func.name());
        unsafe { FUNCTION = func };

        println!("  # 将ra寄存器压栈,保存ra的值");
        println!("  addi sp, sp, -16");
        println!("  sd ra, 8(sp)");

        println!("  # 将fp压栈,fp属于“被调用者保存”的寄存器,需要恢复原值");
        println!("  sd fp, 0(sp)");
        println!("  # 将sp的值写入fp");
        println!("  mv fp, sp");

        println!("  # sp腾出StackSize大小的栈空间");
        println!("  addi sp, sp, -{}", func.stack_size());

        let mut i = 0;
        let mut var = func.params();
        while !var.ptr.is_none() {
            println!("  # 将{}寄存器的值存入{}的栈地址", ARG_REG[i], var.name());
            println!("  sd {}, {}(fp)", ARG_REG[i], var.offset());
            var = var.nxt();
            i += 1;
        }

        println!("\n# =====段主体===============");
        let node = func.body();
        gen_stmt(node);
        assert!(unsafe { DEPTH == 0 });

        println!("\n# =====段结束===============");
        println!("# return段标签");
        println!(".L.return.{}:", func.name());
        println!("  # 将fp的值写回sp");
        println!("  mv sp, fp");
        println!("  # 将最早fp保存的值弹栈,恢复fp和sp");
        println!("  ld fp, 0(sp)");
        // println!("  addi sp, sp, 8");

        println!("  # 将ra寄存器弹栈,恢复ra的值");
        println!("  ld ra, 8(sp)");
        println!("  addi sp, sp, 16");

        println!("  # 返回a0值给系统调用");
        println!("  ret");
        func = func.nxt();
    }
}

#[allow(dead_code)]
pub fn load(ty: TyWrap) {
    if ty.kind() == Some(TypeKind::ARRAY) {
        return;
    }
    println!("  # 读取a0中存放的地址,得到的值存入a0");
    println!("  ld a0, 0(a0)");
}

#[allow(dead_code)]
pub fn store() {
    pop("a1");
    println!("  # 将a0的值,写入到a1中存放的地址");
    println!("  sd a0, 0(a1)");
}

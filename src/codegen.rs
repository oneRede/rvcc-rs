use std::{fs::File, io::Write};

use crate::{
    node::{NodeKind, NodeWrap},
    obj::ObjWrap,
    ty::{TyWrap, TypeKind},
    utils::error_token,
    INPUT_PATH,
};

pub static mut DEPTH: usize = 0;
pub static mut I: i64 = 1;

pub static ARG_REG: [&str; 6] = ["a0", "a1", "a2", "a3", "a4", "a5"];
pub static mut FUNCTION: ObjWrap = ObjWrap::empty();

pub static mut OUTPUT_FILE: Option<File> = None;

#[allow(dead_code)]
pub fn write_to_file(code: &str) {
    let _ = unsafe { OUTPUT_FILE.as_ref().unwrap().write_all(code.as_bytes()) };
    let _ = unsafe { OUTPUT_FILE.as_ref().unwrap().write_all("\n".as_bytes()) };
}

#[allow(dead_code)]
pub fn count() -> i64 {
    unsafe {
        I += 1;
        return I;
    }
}

#[allow(dead_code)]
pub fn push() {
    write_to_file(&format!("  # 压栈,将a0的值存入栈顶"));
    write_to_file(&format!("  addi sp, sp, -8"));
    write_to_file(&format!("  sd a0, 0(sp)"));
    unsafe { DEPTH += 1 };
}

#[allow(dead_code)]
pub fn pop(reg: &str) {
    write_to_file(&format!("  # 弹栈，将栈顶的值存入{}", reg));
    write_to_file(&format!("  ld {}, 0(sp)", reg));
    write_to_file(&format!("  addi sp, sp, 8"));
    unsafe { DEPTH -= 1 };
}

#[allow(dead_code)]
pub fn align_to(n: usize, align: usize) -> usize {
    return (n + align - 1) / align * align;
}

#[allow(dead_code)]
pub fn gen_addr(node: NodeWrap) {
    write_to_file(&format!("  .loc 1 {}", node.token().line_no()));
    match node.kind() {
        NodeKind::VAR => {
            if node.var().is_local() {
                let offset = node.var().offset();
                let name = node.var().name();
                write_to_file(&format!(
                    "  # 获取局部变量{}的栈内地址为{}(fp)",
                    name, offset
                ));
                write_to_file(&format!("  addi a0, fp, {}", offset));
            } else {
                let name = node.var().name();
                write_to_file(&format!("  # 获取全局变量{}的地址", name));
                write_to_file(&format!("  la a0, {}", name));
            }
            return;
        }
        NodeKind::DEREF => {
            gen_expr(node.lhs());
            return;
        }
        NodeKind::COMMA => {
            gen_expr(node.lhs());
            gen_addr(node.rhs());
            return;
        }
        NodeKind::MEMBER => {
            gen_addr(node.lhs());
            write_to_file(&format!("  # 计算成员变量的地址偏移量"));
            write_to_file(&format!("  li t0, {}", node.mem().offset()));
            write_to_file(&format!("  add a0, a0, t0"));
            return;
        }
        _ => {}
    }
    error_token(node.token(), "not an lvalue");
}

#[allow(dead_code)]
pub fn gen_expr(node: NodeWrap) {
    write_to_file(&format!("  .loc 1 {}", node.token().line_no()));
    match node.kind() {
        NodeKind::Num => {
            write_to_file(&format!("  # 将{}加载到a0中", node.val()));
            write_to_file(&format!("  li a0, {}", node.val()));
            return;
        }
        NodeKind::NEG => {
            gen_expr(node.lhs());
            write_to_file(&format!("  # 对a0值进行取反"));
            write_to_file(&format!("  neg a0, a0"));
            return;
        }
        NodeKind::VAR | NodeKind::MEMBER => {
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
            store(node.ty());
            return;
        }
        NodeKind::STMTEXPR => {
            for nd in node.body().into_iter() {
                gen_stmt(nd);
            }
            return;
        }
        NodeKind::COMMA => {
            gen_expr(node.lhs());
            gen_expr(node.rhs());
            return;
        }
        NodeKind::FUNCALL => {
            let mut n_args = 0;

            for nd in node.args() {
                gen_expr(nd);
                push();
                n_args += 1;
            }

            for i in 0..(n_args) {
                pop(ARG_REG[(n_args - 1 - i) as usize]);
            }

            write_to_file(&format!("\n  # 调用函数{}", node.func_name()));
            write_to_file(&format!("  call {}", node.func_name()));
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
            write_to_file(&format!("  # a0+a1,结果写入a0"));
            write_to_file(&format!("  add a0, a0, a1"));
            return;
        }
        NodeKind::Sub => {
            write_to_file(&format!("  # a0-a1,结果写入a0"));
            write_to_file(&format!("  sub a0, a0, a1"));
            return;
        }
        NodeKind::Mul => {
            write_to_file(&format!("  # a0*a1,结果写入a0"));
            write_to_file(&format!("  mul a0, a0, a1"));
            return;
        }
        NodeKind::Div => {
            write_to_file(&format!("  # a0÷a1,结果写入a0"));
            write_to_file(&format!("  div a0, a0, a1"));
            return;
        }
        NodeKind::EQ => {
            write_to_file(&format!("  xor a0, a0, a1"));
            write_to_file(&format!("  seqz a0, a0"));
            return;
        }
        NodeKind::NE => {
            let p = if node.kind() == NodeKind::EQ {
                "="
            } else {
                "!="
            };
            write_to_file(&format!("  # 判断是否a0{}a1", p));
            write_to_file(&format!("  xor a0, a0, a1"));
            write_to_file(&format!("  snez a0, a0"));
            return;
        }
        NodeKind::LT => {
            write_to_file(&format!("  # 判断a0<a1"));
            write_to_file(&format!("  slt a0, a0, a1"));
            return;
        }
        NodeKind::LE => {
            write_to_file(&format!("  # 判断是否a0≤a1"));
            write_to_file(&format!("  slt a0, a1, a0"));
            write_to_file(&format!("  xori a0, a0, 1"));
            return;
        }
        _ => {}
    }
    error_token(node.token(), "invalid expression");
}

#[allow(dead_code)]
fn gen_stmt(node: NodeWrap) {
    match node.kind() {
        NodeKind::IF => {
            let c = count();
            write_to_file(&format!("\n# =====分支语句{}==============", c));
            write_to_file(&format!("\n# Cond表达式{}", c));
            gen_expr(node.cond());
            write_to_file(&format!("  # 若a0为0,则跳转到分支{}的.L.else.{}段", c, c));
            write_to_file(&format!("  beqz a0, .L.else.{}", c));

            write_to_file(&format!("\n# Then语句{}", c));
            gen_stmt(node.then());
            write_to_file(&format!("  # 跳转到分支{}的.L.end.{}段\n", c, c));
            write_to_file(&format!("  j .L.end.{}", c));
            write_to_file(&format!("\n# Else语句{}", c));
            write_to_file(&format!("# 分支{}的.L.else.{}段标签", c, c));
            write_to_file(&format!(".L.else.{}:", c));

            if !node.els().ptr.is_none() {
                gen_stmt(node.els())
            }
            write_to_file(&format!("\n# 分支{}的.L.end.{}段标签", c, c));
            write_to_file(&format!(".L.end.{}:", c));
            return;
        }

        NodeKind::FOR => {
            let c = count();
            write_to_file(&format!("\n# =====循环语句{}===============", c));
            if !node.init().ptr.is_none() {
                write_to_file(&format!("\n# Init语句%{}", c));
                gen_stmt(node.init());
            }

            write_to_file(&format!("\n# 循环{}的.L.begin.{}段标签", c, c));
            write_to_file(&format!(".L.begin.{}:", c));

            write_to_file(&format!("# Cond表达式{}", c));
            if !node.cond().ptr.is_none() {
                gen_expr(node.cond());
                write_to_file(&format!("  # 若a0为0,则跳转到循环{}的.L.end.{}段", c, c));
                write_to_file(&format!("  beqz a0, .L.end.{}", c));
            }

            write_to_file(&format!("\n# Then语句{}", c));
            gen_stmt(node.then());

            if !node.inc().ptr.is_none() {
                write_to_file(&format!("\n# Inc语句{}", c));
                gen_expr(node.inc())
            }

            write_to_file(&format!("  # 跳转到循环{}的.L.begin.{}段", c, c));
            write_to_file(&format!("  j .L.begin.{}", c));
            write_to_file(&format!("\n# 循环{}的.L.end.{}段标签", c, c));
            write_to_file(&format!(".L.end.{}:", c));
            return;
        }

        NodeKind::BLOCK => {
            for nd in node.body() {
                gen_stmt(nd);
            }
            return;
        }

        NodeKind::RETURN => {
            write_to_file(&format!("# 返回语句"));
            gen_expr(node.lhs());
            write_to_file(&format!(
                "  # 跳转到.L.return.{}段",
                unsafe { FUNCTION }.name()
            ));
            write_to_file(&format!("  j .L.return.{}", unsafe { FUNCTION }.name()));
            return;
        }
        NodeKind::EXPRSTMT => {
            gen_expr(node.lhs());
            return;
        }
        _ => {}
    }
    error_token(node.token(), "invalid statement");
}

#[allow(dead_code)]
pub fn assign_l_var_offsets(prog: ObjWrap) {
    for func in prog {
        if !func.is_function() {
            continue;
        }

        let mut offset = 0;
        for obj in func.locals() {
            offset += obj.ty().size();
            offset = align_to(offset, obj.ty().align());
            obj.set_offset(-(offset as i64));
        }
        func.set_stack_size(align_to(offset, 16));
    }
}

#[allow(dead_code)]
pub fn emit_text(prog: ObjWrap) {
    for func in prog {
        if !func.is_function() {
            continue;
        }
        write_to_file(&format!("\n  # 定义全局{}段", func.name()));
        write_to_file(&format!("  .globl {}", func.name()));

        write_to_file(&format!("  # 代码段标签"));
        write_to_file(&format!("  .text"));
        write_to_file(&format!("# ====={}段开始===============", func.name()));
        write_to_file(&format!("# {}段标签", func.name()));
        write_to_file(&format!("{}:", func.name()));
        unsafe { FUNCTION = func };

        write_to_file(&format!("  # 将ra寄存器压栈,保存ra的值"));
        write_to_file(&format!("  addi sp, sp, -16"));
        write_to_file(&format!("  sd ra, 8(sp)"));

        write_to_file(&format!(
            "  # 将fp压栈,fp属于“被调用者保存”的寄存器,需要恢复原值"
        ));
        write_to_file(&format!("  sd fp, 0(sp)"));
        write_to_file(&format!("  # 将sp的值写入fp"));
        write_to_file(&format!("  mv fp, sp"));

        write_to_file(&format!("  # sp腾出StackSize大小的栈空间"));
        write_to_file(&format!("  addi sp, sp, -{}", func.stack_size()));

        let mut i = 0;
        for var in func.params() {
            store_genernal(i, var.offset(), var.ty().size());
            i += 1;
        }

        write_to_file(&format!("\n# =====段主体==============="));
        let node = func.body();
        gen_stmt(node);
        assert!(unsafe { DEPTH == 0 });

        write_to_file(&format!("\n# =====段结束==============="));
        write_to_file(&format!("# return段标签"));
        write_to_file(&format!(".L.return.{}:", func.name()));
        write_to_file(&format!("  # 将fp的值写回sp"));
        write_to_file(&format!("  mv sp, fp"));
        write_to_file(&format!("  # 将最早fp保存的值弹栈,恢复fp和sp"));
        write_to_file(&format!("  ld fp, 0(sp)"));

        write_to_file(&format!("  # 将ra寄存器弹栈,恢复ra的值"));
        write_to_file(&format!("  ld ra, 8(sp)"));
        write_to_file(&format!("  addi sp, sp, 16"));

        write_to_file(&format!("  # 返回a0值给系统调用"));
        write_to_file(&format!("  ret"));
    }
}

#[allow(dead_code)]
pub fn load(ty: TyWrap) {
    if ty.kind() == Some(TypeKind::ARRAY)
        || ty.kind() == Some(TypeKind::STRUCT)
        || ty.kind() == Some(TypeKind::UNION)
    {
        return;
    }
    write_to_file(&format!("  # 读取a0中存放的地址,得到的值存入a0"));
    if ty.size() == 1 {
        write_to_file(&format!("  lb a0, 0(a0)"));
    } else if ty.size() == 4 {
        write_to_file(&format!("  lw a0, 0(a0)"));
    } else {
        write_to_file(&format!("  ld a0, 0(a0)"));
    }
}

#[allow(dead_code)]
pub fn store(ty: TyWrap) {
    pop("a1");

    let kind = if ty.kind() == Some(TypeKind::STRUCT) {
        "结构体"
    } else {
        "联合体"
    };

    if ty.kind() == Some(TypeKind::STRUCT) || ty.kind() == Some(TypeKind::UNION) {
        write_to_file(&format!("  # 对{}进行赋值", kind));
        for i in 0..ty.size() {
            write_to_file(&format!("  li t0, {}", i));
            write_to_file(&format!("  add t0, a0, t0"));
            write_to_file(&format!("  lb t1, 0(t0)"));

            write_to_file(&format!("  li t0, {}", i));
            write_to_file(&format!("  add t0, a1, t0"));
            write_to_file(&format!("  sb t1, 0(t0)"));
        }
        return;
    }

    write_to_file(&format!("  # 将a0的值,写入到a1中存放的地址"));
    if ty.size() == 1 {
        write_to_file(&format!("  sb a0, 0(a1)"));
    } else if ty.size() == 4 {
        write_to_file(&format!("  sw a0, 0(a1)"));
    } else {
        write_to_file(&format!("  sd a0, 0(a1)"));
    }
}

#[allow(dead_code)]
pub fn emit_data(prog: ObjWrap) {
    for var in prog {
        if var.is_function() {
            continue;
        }
        let name = var.name();
        let size = var.ty().size();
        write_to_file(&format!("  # 数据段标签"));
        write_to_file(&format!("  .data"));

        if !var.init_data().is_empty() {
            write_to_file(&format!("{}:", var.name()));
            for c in var.init_data() {
                let n = c;
                if c >= 32 {
                    write_to_file(&format!("  .byte {}\t# 字符：{}", n, n as u8 as char));
                } else {
                    write_to_file(&format!("  .byte {}", n));
                }
            }
            write_to_file(&format!("  .byte {}", 0));
        } else {
            write_to_file(&format!("  # 全局段{}", name));
            write_to_file(&format!("  .globl {}", name));
            write_to_file(&format!("{}:", name));
            write_to_file(&format!("  # 全局变量零填充{}位", size));
            write_to_file(&format!("  .zero {}", size));
        }
    }
}

#[allow(dead_code)]
pub(crate) fn codegen(prog: ObjWrap, out: File) {
    unsafe { OUTPUT_FILE = Some(out) };

    write_to_file(&format!(".file 1 \"{}\"\n", unsafe {
        INPUT_PATH.as_ref().unwrap()
    }));

    assign_l_var_offsets(prog);
    emit_data(prog);
    emit_text(prog);
}

#[allow(dead_code)]
pub fn store_genernal(reg: usize, offset: i64, size: usize) {
    write_to_file(&format!(
        "  # 将{}寄存器的值存入{}(fp)的栈地址",
        ARG_REG[reg], offset
    ));
    match size {
        1 => {
            write_to_file(&format!("  sb {}, {}(fp)", ARG_REG[reg], offset));
            return;
        }
        4 => {
            write_to_file(&format!("  sw {}, {}(fp)", ARG_REG[reg], offset));
            return;
        }
        8 => {
            write_to_file(&format!("  sd {}, {}(fp)", ARG_REG[reg], offset));
            return;
        }
        _ => {
            panic!("unreachable")
        }
    }
}

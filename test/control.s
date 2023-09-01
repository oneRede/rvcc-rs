  # 数据段标签
  .data
.L..1:
  .byte 79	# 字符：O
  .byte 75	# 字符：K
  .byte 10
  .byte 0
  # 数据段标签
  .data
.L..0:
  .byte 40	# 字符：(
  .byte 123	# 字符：{
  .byte 32	# 字符： 
  .byte 105	# 字符：i
  .byte 110	# 字符：n
  .byte 116	# 字符：t
  .byte 32	# 字符： 
  .byte 105	# 字符：i
  .byte 61	# 字符：=
  .byte 48	# 字符：0
  .byte 59	# 字符：;
  .byte 32	# 字符： 
  .byte 105	# 字符：i
  .byte 110	# 字符：n
  .byte 116	# 字符：t
  .byte 32	# 字符： 
  .byte 106	# 字符：j
  .byte 61	# 字符：=
  .byte 48	# 字符：0
  .byte 59	# 字符：;
  .byte 32	# 字符： 
  .byte 102	# 字符：f
  .byte 111	# 字符：o
  .byte 114	# 字符：r
  .byte 32	# 字符： 
  .byte 40	# 字符：(
  .byte 105	# 字符：i
  .byte 61	# 字符：=
  .byte 48	# 字符：0
  .byte 59	# 字符：;
  .byte 32	# 字符： 
  .byte 105	# 字符：i
  .byte 60	# 字符：<
  .byte 61	# 字符：=
  .byte 49	# 字符：1
  .byte 48	# 字符：0
  .byte 59	# 字符：;
  .byte 32	# 字符： 
  .byte 105	# 字符：i
  .byte 61	# 字符：=
  .byte 105	# 字符：i
  .byte 43	# 字符：+
  .byte 49	# 字符：1
  .byte 41	# 字符：)
  .byte 32	# 字符： 
  .byte 106	# 字符：j
  .byte 61	# 字符：=
  .byte 105	# 字符：i
  .byte 43	# 字符：+
  .byte 106	# 字符：j
  .byte 59	# 字符：;
  .byte 32	# 字符： 
  .byte 106	# 字符：j
  .byte 59	# 字符：;
  .byte 32	# 字符： 
  .byte 125	# 字符：}
  .byte 41	# 字符：)
  .byte 0

  # 定义全局main段
  .globl main
  # 代码段标签
  .text
# =====main段开始===============
# main段标签
main:
  # 将ra寄存器压栈,保存ra的值
  addi sp, sp, -16
  sd ra, 8(sp)
  # 将fp压栈,fp属于“被调用者保存”的寄存器,需要恢复原值
  sd fp, 0(sp)
  # 将sp的值写入fp
  mv fp, sp
  # sp腾出StackSize大小的栈空间
  addi sp, sp, -0

# =====段主体===============
  # 将55加载到a0中
  li a0, 55
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 获取局部变量i的栈内地址为0(fp)
  addi a0, fp, 0
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 将0加载到a0中
  li a0, 0
  # 弹栈，将栈顶的值存入a1
  ld a1, 0(sp)
  addi sp, sp, 8
  # 将a0的值,写入到a1中存放的地址
  sd a0, 0(a1)
  # 获取局部变量j的栈内地址为0(fp)
  addi a0, fp, 0
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 将0加载到a0中
  li a0, 0
  # 弹栈，将栈顶的值存入a1
  ld a1, 0(sp)
  addi sp, sp, 8
  # 将a0的值,写入到a1中存放的地址
  sd a0, 0(a1)

# =====循环语句2===============

# Init语句%2
  # 获取局部变量i的栈内地址为0(fp)
  addi a0, fp, 0
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 将0加载到a0中
  li a0, 0
  # 弹栈，将栈顶的值存入a1
  ld a1, 0(sp)
  addi sp, sp, 8
  # 将a0的值,写入到a1中存放的地址
  sd a0, 0(a1)

# 循环2的.L.begin.2段标签
.L.begin.2:
# Cond表达式2
  # 将10加载到a0中
  li a0, 10
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 获取局部变量i的栈内地址为0(fp)
  addi a0, fp, 0
  # 读取a0中存放的地址,得到的值存入a0
  ld a0, 0(a0)
  # 弹栈，将栈顶的值存入a1
  ld a1, 0(sp)
  addi sp, sp, 8
  # 判断是否a0≤a1
  slt a0, a1, a0
  xori a0, a0, 1
  # 若a0为0,则跳转到循环2的.L.end.2段
  beqz a0, .L.end.2

# Then语句2
  # 获取局部变量j的栈内地址为0(fp)
  addi a0, fp, 0
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 获取局部变量j的栈内地址为0(fp)
  addi a0, fp, 0
  # 读取a0中存放的地址,得到的值存入a0
  ld a0, 0(a0)
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 获取局部变量i的栈内地址为0(fp)
  addi a0, fp, 0
  # 读取a0中存放的地址,得到的值存入a0
  ld a0, 0(a0)
  # 弹栈，将栈顶的值存入a1
  ld a1, 0(sp)
  addi sp, sp, 8
  # a0+a1,结果写入a0
  add a0, a0, a1
  # 弹栈，将栈顶的值存入a1
  ld a1, 0(sp)
  addi sp, sp, 8
  # 将a0的值,写入到a1中存放的地址
  sd a0, 0(a1)

# Inc语句2
  # 获取局部变量i的栈内地址为0(fp)
  addi a0, fp, 0
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 将1加载到a0中
  li a0, 1
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 获取局部变量i的栈内地址为0(fp)
  addi a0, fp, 0
  # 读取a0中存放的地址,得到的值存入a0
  ld a0, 0(a0)
  # 弹栈，将栈顶的值存入a1
  ld a1, 0(sp)
  addi sp, sp, 8
  # a0+a1,结果写入a0
  add a0, a0, a1
  # 弹栈，将栈顶的值存入a1
  ld a1, 0(sp)
  addi sp, sp, 8
  # 将a0的值,写入到a1中存放的地址
  sd a0, 0(a1)
  # 跳转到循环2的.L.begin.2段
  j .L.begin.2

# 循环2的.L.end.2段标签
.L.end.2:
  # 获取局部变量j的栈内地址为0(fp)
  addi a0, fp, 0
  # 读取a0中存放的地址,得到的值存入a0
  ld a0, 0(a0)
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 获取全局变量.L..0的地址
  la a0, .L..0
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 弹栈，将栈顶的值存入a2
  ld a2, 0(sp)
  addi sp, sp, 8
  # 弹栈，将栈顶的值存入a1
  ld a1, 0(sp)
  addi sp, sp, 8
  # 弹栈，将栈顶的值存入a0
  ld a0, 0(sp)
  addi sp, sp, 8

  # 调用函数assert
  call assert
  # 获取全局变量.L..1的地址
  la a0, .L..1
  # 压栈,将a0的值存入栈顶
  addi sp, sp, -8
  sd a0, 0(sp)
  # 弹栈，将栈顶的值存入a0
  ld a0, 0(sp)
  addi sp, sp, 8

  # 调用函数printf
  call printf
# 返回语句
  # 将0加载到a0中
  li a0, 0
  # 跳转到.L.return.main段
  j .L.return.main

# =====段结束===============
# return段标签
.L.return.main:
  # 将fp的值写回sp
  mv sp, fp
  # 将最早fp保存的值弹栈,恢复fp和sp
  ld fp, 0(sp)
  # 将ra寄存器弹栈,恢复ra的值
  ld ra, 8(sp)
  addi sp, sp, 16
  # 返回a0值给系统调用
  ret

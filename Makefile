# test/文件夹的c测试文件
TEST_SRCS=$(wildcard test/*.c)
# test/文件夹的c测试文件编译出的可执行文件
TESTS=$(TEST_SRCS:.c=.exe)

# rvcc标签，表示如何构建最终的二进制文件
rvcc:
# 将多个*.o文件编译为rvcc
	cargo build

# 测试标签，运行测试
test/%.exe: rvcc test/%.c
# $(CC) -o- -E -P -C test/$*.c | ./rvcc -o test/$*.s -
	/usr/bin/riscv64-linux-gnu-gcc -o- -E -P -C test/$*.c | ./target/debug/rvcc -o test/$*.s -
# $(CC) -o $@ test/$*.s -xc test/common
	/usr/bin/riscv64-linux-gnu-gcc -static -o $@ test/$*.s -xc test/common

test: $(TESTS)
#   for i in $^; do echo $$i; ./$$i || exit 1; echo; done
	for i in $^; do echo $$i; /usr/bin/qemu-riscv64 -L $(RISCV)/sysroot ./$$i || exit 1; echo; done
#	for i in $^; do echo $$i; $(RISCV)/bin/spike --isa=rv64gc $(RISCV)/riscv64-unknown-linux-gnu/bin/pk ./$$i || exit 1; echo; done
	test/driver.sh

# 清理标签，清理所有非源代码文件
clean:
	cargo clean
	rm -rf rvcc tmp* $(TESTS) test/*.s test/*.exe
	rm -rf *.s tmp *.o

# 伪目标，没有实际的依赖文件
.PHONY: test clean
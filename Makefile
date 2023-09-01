# rvcc标签，表示如何构建最终的二进制文件
rvcc:
# 将多个*.o文件编译为rvcc
	cargo build

# 测试标签，运行测试脚本
test: rvcc
	./test.sh
	./test-driver.sh

# 清理标签，清理所有非源代码文件
clean:
	cargo clean
	rm -rf *.s tmp *.o

# 伪目标，没有实际的依赖文件
.PHONY: test clean
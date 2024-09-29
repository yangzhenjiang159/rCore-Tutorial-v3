# 定义目标平台
TARGET := riscv64gc-unknown-none-elf
# 定义构建模式，默认为 release
MODE := release
# 定义内核 ELF 文件路径
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
# 定义内核二进制文件路径
KERNEL_BIN := $(KERNEL_ELF).bin
# 定义反汇编临时文件路径
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

# 根据构建模式设置参数
ifeq ($(MODE), release)
	# 如果是 release 模式，设置 --release 参数
	MODE_ARG := --release
endif

# 定义开发板类型
BOARD := qemu
# 定义 SBI 类型，默认为 rustsbi
SBI?= rustsbi
# 定义引导加载程序文件路径
BOOTLOADER :=../bootloader/$(SBI)-$(BOARD).bin

# 定义内核入口物理地址
KERNEL_ENTRY_PA := 0x80200000

# 定义 binutils 工具
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# 定义反汇编参数
DISASM?= -x

# 构建目标依赖
build: env $(KERNEL_BIN)

# 环境准备目标
env:
	# 检查是否安装了目标平台，如果没有则安装
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	# 安装 cargo-binutils
	cargo install cargo-binutils
	# 安装 rust-src 组件
	rustup component add rust-src
	# 安装 llvm-tools-preview 组件
	rustup component add llvm-tools-preview

# 生成内核二进制文件
$(KERNEL_BIN): kernel
	# 使用 objcopy 将内核 ELF 文件转换为二进制文件
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

# 构建内核
kernel:
	# 打印当前平台信息
	@echo Platform: $(BOARD)
	# 复制特定平台的链接器脚本
	@cp src/linker-$(BOARD).ld src/linker.ld
	# 使用 cargo 构建内核
	@cargo build $(MODE_ARG)
	# 删除临时链接器脚本
	@rm src/linker.ld

# 清理目标
clean:
	# 使用 cargo 清理构建产物
	@cargo clean

# 反汇编内核
disasm: kernel
	# 使用 objdump 反汇编内核 ELF 文件
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

# 使用 vim 查看反汇编代码
disasm-vim: kernel
	# 将反汇编结果输出到临时文件
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	# 使用 vim 打开临时文件
	@vim $(DISASM_TMP)
	# 删除临时文件
	@rm $(DISASM_TMP)

# 运行目标
run: run-inner

# QEMU 参数
QEMU_ARGS := -machine virt \
			 -nographic \
			 -bios $(BOOTLOADER) \
			 -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

# 默认 QEMU 名称
QEMU_NAME := qemu-system-riscv64
# QEMU 版本检查目标
qemu-version-check:
	# 运行脚本检查 QEMU 版本
	@sh scripts/qemu-ver-check.sh $(QEMU_NAME)

# 内部运行目标
run-inner: qemu-version-check build
	# 运行 QEMU
	@qemu-system-riscv64 $(QEMU_ARGS)

# 调试目标
debug: qemu-version-check build
	# 在 tmux 中启动 QEMU 并连接 GDB
	@tmux new-session -d \
		"qemu-system-riscv64 $(QEMU_ARGS) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

# GDB 服务器目标
gdbserver: qemu-version-check build
	# 启动 QEMU 并监听 GDB 连接
	@qemu-system-riscv64 $(QEMU_ARGS) -s -S

# GDB 客户端目标
gdbclient:
	# 使用 GDB 连接到远程 QEMU 服务器
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

# 伪目标
.PHONY: build env kernel clean disasm disasm-vim run-inner gdbserver gdbclient qemu-version-check

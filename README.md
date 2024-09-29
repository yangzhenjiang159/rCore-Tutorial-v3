# rCore-Tutorial-v3
rCore-Tutorial version 3.6. 查看 中文文档](https://rcore-os.github.io/rCore-Tutorial-Book-v3/).

rCore-Tutorial API 文档.  查看 [十个操作系统的API文档](#OS-API-DOCS)

官方QQ群: 735045051

## news
- 23/06/2022：版本 3.6.0 正在制作中！现在我们直接在 chX 分支上更新代码，请定期检查是否有更新。

## 概述

该项目旨在向没有任何**计算机体系结构、汇编语言或操作系统**背景知识的**初学者**展示如何**从头开始**用**[Rust](https://www.rust-lang.org/)**编写一个运行在**RISC-V**平台上的**类 Unix 操作系统**。

## 特性

* 平台支持：`qemu-system-riscv64` 模拟器或基于 [Kendryte K210 SoC](https://canaan.io/product/kendryteai) 的开发板，如 [Maix Dock](https://www.seeedstudio.com/Sipeed-MAIX-Dock-p-4815.html)
* 操作系统特性：
  * 支持多进程并发，每个进程包含多个原生线程
  * 抢占式调度（Round-Robin 算法）
  * 内核中的动态内存管理
  * 虚拟内存
  * 带有块缓存的简单文件系统
  * 用户空间中的交互式 shell
* **仅 4K+ LoC**
* 尽管代码中缺乏注释，但有[详细的中文文档](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)（目前暂无英文版本）

## 环境安装

### 安装 Rust

请参阅 [官方指南](https://www.rust-lang.org/tools/install).

安装一些工具：

```sh
$ rustup target add riscv64gc-unknown-none-elf
$ cargo install cargo-binutils --vers =0.3.3
$ rustup component add llvm-tools-preview
$ rustup component add rust-src
```

### 安装 Qemu

在这里，我们手动编译并安装Qemu 7.0.0。例如，在Ubuntu 18.04上：

```sh
# 安装依赖包
$ sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3 python3-pip
# 下载 Qemu 源代码
$ wget https://download.qemu.org/qemu-7.0.0.tar.xz
# 解压到 qemu-7.0.0/
$ tar xvJf qemu-7.0.0.tar.xz
$ cd qemu-7.0.0
# 配置 并编译
$ ./configure --target-list=riscv64-softmmu,riscv64-linux-user
$ make -j$(nproc)
```

然后，将以下内容添加到 ~/.bashrc 中（请根据您的环境调整这些路径）：

```
export PATH=$PATH:/home/shinbokuow/Downloads/built/qemu-7.0.0
export PATH=$PATH:/home/shinbokuow/Downloads/built/qemu-7.0.0/riscv64-softmmu
export PATH=$PATH:/home/shinbokuow/Downloads/built/qemu-7.0.0/riscv64-linux-user
```

最后，更新当前 shell：

```sh
$ source ~/.bashrc
```
现在我们可以检查 Qemu 的版本：

```sh
$ qemu-system-riscv64 --version
QEMU emulator version 7.0.0
Copyright (c) 2003-2020 Fabrice Bellard and the QEMU Project developers
```

### 安装 RISC-V GNU 嵌入式工具链（包括 GDB）

从 [Sifive 网站](https://www.sifive.com/software)（Ctrl+F 'toolchain'）根据你的平台下载压缩文件。

解压并将其根目录下的 'bin' 目录位置添加到 `$PATH`。

例如，我们可以检查 GDB 的版本：

```sh
$ riscv64-unknown-elf-gdb --version
GNU gdb (SiFive GDB-Metal 10.1.0-2020.12.7) 10.1
Copyright (C) 2020 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
```

### 安装串口工具（可选，如果想在 K210 上运行）

```sh
$ pip3 install pyserial
$ sudo apt install python3-serial
```

## 运行我们的项目

### Qemu

```sh
$ git clone https://github.com/rcore-os/rCore-Tutorial-v3.git
$ cd rCore-Tutorial-v3/os
$ make run
```

在输出一些调试信息后，内核会列出所有可用的应用程序并进入用户 shell：

```
/**** APPS ****
mpsc_sem
usertests
pipetest
forktest2
cat
initproc
race_adder_loop
threads_arg
race_adder_mutex_spin
race_adder_mutex_blocking
forktree
user_shell
huge_write
race_adder
race_adder_atomic
threads
stack_overflow
filetest_simple
forktest_simple
cmdline_args
run_pipe_test
forktest
matrix
exit
fantastic_text
sleep_simple
yield
hello_world
pipe_large_test
sleep
phil_din_mutex
**************/
Rust user shell
>> 
```

你可以运行除 initproc 和 user_shell 之外的任何应用程序。要运行一个应用程序，只需输入其文件名并按回车键。usertests 可以运行一堆应用程序，因此推荐使用。

输入 Ctrl+a 然后 x 退出 Qemu。

### K210

在第 6 章之前，你不需要 SD 卡：

```sh
$ git clone https://github.com/rcore-os/rCore-Tutorial-v3.git
$ cd rCore-Tutorial-v3/os
$ make run BOARD=k210
```

从第 6 章开始，在运行内核之前，我们应该将 SD 卡插入 PC 并手动将文件系统映像写入其中：

```sh
$ cd rCore-Tutorial-v3/os
$ make sdcard
```

默认情况下，它将覆盖 SD 卡的设备 /dev/sdb，但你可以提供另一个位置。例如，make sdcard SDCARD=/dev/sdc。

之后，从 PC 中取出 SD 卡并将其插入 K210 的插槽中。将 K210 连接到 PC，然后：

```sh
$ git clone https://github.com/rcore-os/rCore-Tutorial-v3.git
$ cd rCore-Tutorial-v3/os
$ make run BOARD=k210
```

输入 Ctrl+] 从 K210 断开连接。

## Rustdoc

目前它只能帮助你查看代码，因为只有很小一部分代码被记录下来。

你可以在 os 目录下使用 cargo doc --no-deps --open 打开 os 的文档 html。

rCore-Tutorial 版本 3.6。查看 [中文文档](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)。

rCore-Tutorial API 文档。查看 [十个操作系统的 API 文档](#OS-API-DOCS)。

官方 QQ 群号：735045051
### OS-API-DOCS
十个操作系统的 API 文档
1. [Lib-OS API doc](https://learningos.github.io/rCore-Tutorial-v3/ch1/os/index.html)
1. [Batch-OS API doc](https://learningos.github.io/rCore-Tutorial-v3/ch2/os/index.html)
1. [MultiProg-OS API doc](https://learningos.github.io/rCore-Tutorial-v3/ch3-coop/os/index.html)
1. [TimeSharing-OS API doc](https://learningos.github.io/rCore-Tutorial-v3/ch3/os/index.html)
1. [AddrSpace-OS API doc](https://learningos.github.io/rCore-Tutorial-v3/ch4/os/index.html)
1. [Process-OS API doc](https://learningos.github.io/rCore-Tutorial-v3/ch5/os/index.html)
1. [FileSystem-OS API doc](https://learningos.github.io/rCore-Tutorial-v3/ch6/os/index.html)
1. [IPC-OS API doc](https://learningos.github.io/rCore-Tutorial-v3/ch7/os/index.html)
1. [SyncMutex-OS API doc](https://learningos.github.io/rCore-Tutorial-v3/ch8/os/index.html)
1. [IODevice-OS API doc](https://learningos.github.io/rCore-Tutorial-v3/ch9/os/index.html)
## 正在进行中的工作

我们的第一个版本 3.6.0（第 1-9 章）已经发布，我们仍在努力完善它。

* 第 9 章：需要更多关于不同 I/O 设备的描述

以下是自 3.5.0 以来的更新：

### 已完成

* [x] 在不同平台上运行我们的项目之前，自动清理并重建
* [x] 修复早期章节中的 `power` 系列应用程序，现在你可以在输出中找到模数
* [x] 使用 `UPSafeCell` 代替 `RefCell` 或 `spin::Mutex`，以便访问静态数据结构，并调整其 API，使其不能同时被借用两次（在 `run_first_task` 中提到 `&.exclusive_access().task[0]`）
* [x] 将 `TaskContext` 移动到 `TaskControlBlock` 中，而不是在内核堆栈上原地恢复，从而消除了烦人的 `task_cx_ptr2`
* [x] 用 `asm!` 替换 `llvm_asm!`
* [x] 将 `rcore-fs-fuse` 生成的 fs 镜像大小扩展到 128MiB
* [x] 添加一个名为 `huge_write` 的新测试，用于评估 fs 性能（qemu ~500KiB/s k210 ~50KiB/s）
* [x] 在涉及写操作的 fs 事务后，将所有块缓存刷新到磁盘
* [x] 在 SMP 章节之前，用 `UPSafeCell` 替换 `spin::Mutex`
* [x] 添加关于同步和互斥的新章节代码（仅适用于单处理器）
* [x] bug 修复：在 `PageTable::unmap` 中，我们应该调用 `find_pte` 而不是 `find_pte_create`
* [x] 澄清：“在 `find_pte` 中检查第 3 级 pte 的有效性，而不是在该函数之外检查”不应该是一个 bug
* [x] 第 8 章代码：单处理器上的同步
* [x] 交换第 6 章和第 7 章的代码
* [x] 在第 7/8 章中支持信号机制（仅适用于单线程应用程序）
* [x] 添加 boards/ 目录并支持 rustdoc，例如，你可以使用 `cargo doc --no-deps --open` 查看 crate 的文档
* [x] 第 9 章代码：基于中断的设备驱动程序，包括 UART、块、键盘、鼠标、gpu 设备
* [x] 在 github 中添加 CI 自动测试和文档

### 待办事项（高优先级）

* [ ] 审查文档，当前进度：8/9
* [ ] 可选地使用旧的 fs 镜像，不要总是重建镜像
* [ ] shell 功能改进（待续...）
* [ ] 为每个非零进程退出代码赋予一个唯一且清晰的错误类型
* [ ] 有效的 mm 模块错误处理
* [ ] 添加更多的操作系统函数，以帮助理解操作系统的概念和原理

### 待办事项（低优先级）

* [ ] 重写实践文档，删除一些不恰当的问题
* [ ] 提供在 Rust 源代码级别上的流畅调试体验
* [ ] 使用官方工具格式化代码

### Crates

我们将稍后添加它们。

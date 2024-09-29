# syntax=docker/dockerfile:1

# Stage 1 Build QEMU
# - https://www.qemu.org/download/
# - https://wiki.qemu.org/Hosts/Linux#Building_QEMU_for_Linux
# - https://wiki.qemu.org/Documentation/Platforms/RISCV

# 从 ubuntu:20.04 基础镜像构建 QEMU
FROM ubuntu:20.04 as build_qemu

# 指定 QEMU 版本
ARG QEMU_VERSION=7.0.0

# 更新软件源并安装依赖
RUN sed -i 's/archive.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list && \
    sed -i 's/security.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y wget build-essential libglib2.0-dev libfdt-dev libpixman-1-dev zlib1g-dev ninja-build

# 下载并解压 QEMU 源码
RUN wget https://download.qemu.org/qemu-${QEMU_VERSION}.tar.xz && \
    tar xf qemu-${QEMU_VERSION}.tar.xz && \
    cd qemu-${QEMU_VERSION} && \
    # 配置并编译安装 QEMU
   ./configure --target-list=riscv64-softmmu,riscv64-linux-user && \
    make -j$(nproc) && \
    make install

# Stage 2 Set Lab Environment
# 从 ubuntu:20.04 基础镜像构建实验环境
FROM ubuntu:20.04 as build

# 设置工作目录
WORKDIR /tmp

# 2.0. 安装通用工具
RUN sed -i 's/archive.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y jq curl git python3 wget build-essential \
    # qemu 依赖
    libglib2.0-0 libfdt1 libpixman-1-0 zlib1g \
    # gdb
    gdb-multiarch

# 2.1. 复制 QEMU
COPY --from=build_qemu /usr/local/bin/* /usr/local/bin

# 2.2. 安装 Rust
# - https://www.rust-lang.org/tools/install
# 设置 Rust 环境变量
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static \
    RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
# 安装 Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --no-modify-path --profile minimal --default-toolchain nightly

# 2.3. 构建实验环境
# 见 os/Makefile `env:` 示例。
# 这样可以避免每次使用新容器时都要等待这些步骤。
# 复制 rust-toolchain.toml 文件
COPY rust-toolchain.toml rust-toolchain.toml
# 安装 Rust 目标和工具
RUN rustup target add riscv64gc-unknown-none-elf && \
    cargo install toml-cli cargo-binutils && \
    # 获取 Rust 版本
    RUST_VERSION=$(toml get -r rust-toolchain.toml toolchain.channel) && \
    # 获取组件列表
    Components=$(toml get -r rust-toolchain.toml toolchain.components | jq -r 'join(" ")') && \
    # 安装 Rust 版本和组件
    rustup install $RUST_VERSION && \
    rustup component add --toolchain $RUST_VERSION $Components

# 2.4. 设置 GDB
# 创建符号链接，以便使用 riscv64-unknown-elf-gdb
RUN ln -s /usr/bin/gdb-multiarch /usr/bin/riscv64-unknown-elf-gdb

# Stage 3 Sanity checking
# 从 build 阶段构建测试环境
FROM build as test
# 运行 QEMU 和 Rust 工具的版本检查
RUN qemu-system-riscv64 --version && \
    qemu-riscv64 --version && \
    rustup --version && \
    cargo --version && \
    rustc --version && \
    riscv64-unknown-elf-gdb --version

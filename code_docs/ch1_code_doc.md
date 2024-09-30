./os/src
Rust        4 Files   119 Lines
Assembly    1 Files    11 Lines
├── .devcontainer 
│   └── devcontainer.json 项目的构建过程、环境配置以及开发工具的设置。
├── .vscode
│   └── settings.json 确保 Rust 项目在使用特定的 RISC-V 目标平台时，能够正确编译并且避免一些常见的错误，例如在没有标准库的情况下找不到 test crate 的错误。同时，它也配置了 Rust Analyzer 插件，使其能够正确地理解和分析项目的代码。

├── bootloader(内核依赖的运行在 M 特权级的 SBI 实现，本项目中我们使用 RustSBI)
│   └── rustsbi-qemu.bin(可运行在 qemu 虚拟机上的预编译二进制版本)
├── LICENSE
├── os(我们的内核实现放在 os 目录下)
│   ├── Cargo.toml(内核实现的一些配置文件)
│   ├── Makefile
│   └── src(所有内核的源代码放在 os/src 目录下)
│       ├── console.rs(将打印字符的 SBI 接口进一步封装实现更加强大的格式化输出)
│       ├── entry.asm(设置内核执行环境的的一段汇编代码)
│       ├── lang_items.rs(需要我们提供给 Rust 编译器的一些语义项，目前包含内核 panic 时的处理逻辑)
│       ├── linker-qemu.ld(控制内核内存布局的链接脚本以使内核运行在 qemu 虚拟机上)
│       ├── main.rs(内核主函数)
│       └── sbi.rs(调用底层 SBI 实现提供的 SBI 接口)
├── README.md
└── rust-toolchain(控制整个项目的工具链版本)

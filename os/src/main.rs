//! 主模块和入口点
//! 内核的各种设施被实现为子模块。最重要的是:
//!
//! - [`trap`]:内核的各种设施被实现为子模块。
//! - [`syscall`]:系统调用处理与实现
//!
//! 操作系统也在这个模块中启动。内核代码启动从'entry. asm'执行，
//! 之后调用['rust_main（）']初始化各种功能。（见其源代码详细信息。）
//! 然后我们调用 ['::run_next_app（）'] 并第一次转到用户空间。
#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

use log::*;
#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;
mod sync;
pub mod syscall;
pub mod trap;
mod config;
mod loader;
pub mod task;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    // batch::init();
    // batch::run_next_app();
}

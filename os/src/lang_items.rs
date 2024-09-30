//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;
use log::*;

/// 移除标准库后,必须实现的函数, 用于打印panic相关信息并执行后续动作
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "[kernel] Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("[kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown(true)
}

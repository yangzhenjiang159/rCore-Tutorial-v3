//! Trap 处理
//!
//! 对于 rCore，我们有一个陷阱切入点，即 __alltraps 在 ['init（）'] 中初始化，我们设置'stvec'CSR 指向它。
//! 所有的陷阱都经过__alltraps，这是在 trap. S 中定义的汇编语言代码做了足够的工作来恢复内核空间上下文，
//! 确保 Rust 代码安全运行，并将控制权转移到['trap_handler()'].
//! 然后它根据异常的确切内容调用不同的功能。例如，定时器中断触发任务抢占，系统调用转到 ['syscall()']。

mod context;


use crate::batch::run_next_app;
use crate::syscall::syscall;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

global_asm!(include_str!("trap.S"));

/// initialize CSR `stvec` as the entry of `__alltraps`
/// 初始化CSR的中断处理标识位,发生中断时可以跳到该入口进行处理
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
/// handle an interrupt, exception, or system call from user space
/// 处理来自用户空间的中断、异常或系统调用
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}

pub use context::TrapContext;
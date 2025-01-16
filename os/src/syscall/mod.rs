//! 系统调用的实现
//!
//! 所有系统调用的单个切入点['syscall()']被调用
//! 每当用户空间希望使用“ecall”执行系统调用时
//! 指令。在这种情况下，环境调用异常，作为中的情况之一处理
//! ['crate::trap::trap_handler'].
//!
//! 为清楚起见，每个系统调用都实现为自己的函数，名为
//! 'sys_'然后是系统调用的名称。你可以在
//! 子模块，你也应该这样实现系统调用。

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

mod fs;
mod process;

use fs::*;
use process::*;

/// 使用syscall_id和其他参数处理系统调用异常
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
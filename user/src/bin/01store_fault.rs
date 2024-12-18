#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    //在测试store_fault中，我们将插入一个无效的存储操作…
    println!("Into Test store_fault, we will insert an invalid store operation...");
    //内核应该杀死这个应用程序！
    println!("Kernel should kill this application!");
    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0);
    }
    0
}

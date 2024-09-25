/*
 * The main module
 */

#![no_main]
#![no_std]
mod lang_items;
mod sbi;

#[macro_use]
mod console;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    // 遍历BSS段并逐地址写 0
    (sbss as usize..ebss as usize)
        .for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("hello, world!");
    error!("hello, error!");
    info!("hello, info!");
    panic!("Shutdown machine!");
}

/*
 * The main module
 */
#![no_main]
#![no_std]
#[macro_use]
mod console;
pub mod config;
mod lang_items;
mod loader;
mod logging;
mod sbi;
mod stack_trace;
mod sync;
pub mod syscall;
pub mod task;
pub mod trap;

use core::arch::global_asm;
use log::*;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

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
    logging::init();
    info!("[kernel] hello, world!");
    trap::init();
    loader::load_apps();
    task::run_first_task();
    panic!("unreachable in rust_main!");
    // batch::init();
    // batch::run_next_app();
}

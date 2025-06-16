/*!
 *  The main module
 */
// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
mod console;
pub mod config;
mod lang_items;
mod loader;
mod logging;
mod mm;
mod sbi;
mod stack_trace;
mod sync;
pub mod syscall;
pub mod task;
mod timer;
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
    mm::init();
    info!("[kernel] mm inited!");
    mm::test();
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    // 启动首个应用程序
    task::run_first_task();
    panic!("unreachable in rust_main!");
    // batch::init();
    // batch::run_next_app();
}

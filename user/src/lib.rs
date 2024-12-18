#![no_std]
#![feature(linkage)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    // 遍历BSS段并逐地址写 0
    (start_bss as usize..end_bss as usize)
        .for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

// user/src/bin/03priv_inst.rs
#![no_std]
#![no_main]

use core::arch::asm;
#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Try to execute privileged instruction in U Mode");
    println!("Kernel should kiil this application!");
    unsafe {
        asm!("sret");
    }
    0
}

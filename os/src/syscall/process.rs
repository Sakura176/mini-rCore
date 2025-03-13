use log::debug;

// os/src/syscall/process.rs
use crate::println;
use crate::task::{suspend_current_and_run_next, exit_current_and_run_next};

pub fn sys_yield() -> isize {
    debug!("os sys_yield");
    suspend_current_and_run_next();
    0
}

pub fn sys_exit(xstate: i32) -> ! {
    println!("[kernel] Application exited with code {}", xstate);
    exit_current_and_run_next();
    panic!("unreachable in sys_exit!");
}

use log::{trace, warn};

use crate::task::{suspend_current_and_run_next, exit_current_and_run_next};
use crate::timer::get_time_ms;

pub fn sys_yield() -> isize {
    trace!("os sys_yield");
    suspend_current_and_run_next();
    0
}

pub fn sys_exit(xstate: i32) -> ! {
    warn!("[kernel] Application exited with code {}", xstate);
    exit_current_and_run_next();
    panic!("unreachable in sys_exit!");
}

pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

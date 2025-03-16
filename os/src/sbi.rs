#![allow(unused)]

// 打印字符，参数为usize，将char类型转为usize即可
pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

/*
* @brief 关闭程序, 发散函数
* @param failure 表示程序是否正常退出
* */
pub fn shutdown(failure: bool) -> ! /* ! 表示发散函数，不返回任何值 */ {
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}

pub fn set_timer(timer: usize) {
    sbi_rt::set_timer(timer as _);
}

use core::{arch::asm, ptr};

pub unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;
    // 嵌入汇编指令，将栈帧地址存入fp寄存器
    // rust指令，将寄存器地址存入到fp变量中
    asm!("mv {}, fp", out(reg) fp);

    println!("== Begin stack trace ==");
    while fp != ptr::null() {
        // 获取返回地址
        let saved_ra = *fp.sub(1);
        // 获取前一帧地址
        let saved_fp = *fp.sub(2);

        println!("0x{:016x}, fp = 0x{:016x}", saved_ra, saved_fp);

        fp = saved_fp as *const usize;
    }
    println!("== End stack trace ==");
}

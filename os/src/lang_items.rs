use crate::{error, sbi::shutdown, stack_trace::print_stack_trace};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 获取返回的崩溃信息位置
    if let Some(location) = info.location() {
        // 有返回值，则打印相应信息
        error!(
            "Paniced at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        // 无值，仅打印崩溃信息
        error!("[kernel] Paniced: {}", info.message());
    }
    unsafe { print_stack_trace();}
    shutdown(true);
}

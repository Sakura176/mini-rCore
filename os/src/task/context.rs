#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,      // 被调用者调用函数时保存，调用之前修改，调用之后恢复
    sp: usize,      // 栈指针寄存器，指向下一个要被存储的栈顶位置
    s: [usize; 12], // 临时寄存器
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self { ra: 0, sp: 0, s: [0; 12] }
    }

    pub fn goto_restore(kstack_ptr: usize) -> Self {
        unsafe extern "C" {
            unsafe fn __restore();
        }
        Self { ra: __restore as usize, sp: kstack_ptr, s: [0; 12] }
    }
}

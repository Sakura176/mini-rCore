use crate::config::*;
use crate::trap::TrapContext;
use core::{arch::asm, u8, usize};

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    fn push_context(&self, cx: TrapContext) -> usize {
        let trap_cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *trap_cx_ptr = cx;
        }
        trap_cx_ptr as usize
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] =
    [KernelStack { data: [0; KERNEL_STACK_SIZE] }; MAX_APP_NUM];
static USER_STACK: [UserStack; MAX_APP_NUM] =
    [UserStack { data: [0; USER_STACK_SIZE] }; MAX_APP_NUM];

pub fn get_num_app() -> usize {
    // 获取link_app.S中_num_app 数值
    unsafe extern "C" {
        safe fn _num_app();
    }
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

/// get app info with entry and sp and save 'TrapContext' in kernel stack
pub fn init_app_cx(app_id: usize) -> usize {
    KERNEL_STACK[app_id].push_context(TrapContext::app_init_context(
        get_base_i(app_id),
        USER_STACK[app_id].get_sp(),
    ))
}

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

pub fn load_apps() {
    unsafe extern "C" {
        safe fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start =
        unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };

    for i in 0..num_app {
        let base_i = get_base_i(i);
        // clear region
        (base_i..base_i + APP_SIZE_LIMIT).for_each(|addr| unsafe {
            (addr as *mut u8).write_volatile(00);
        });

        let src = unsafe {
            core::slice::from_raw_parts(
                app_start[i] as *const u8,
                app_start[i + 1] - app_start[i],
            )
        };
        let dst = unsafe {
            core::slice::from_raw_parts_mut(base_i as *mut u8, src.len())
        };
        dst.copy_from_slice(src);
    }
    unsafe {
        asm!("fence.i");
    }
}

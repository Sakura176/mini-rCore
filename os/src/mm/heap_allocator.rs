// os/src/mm/heap_allocator.rs

use core::ptr;

use crate::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;
use log::{debug, info};

// TODO: 泛型参数32作用未知
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

// 定义一个u8类型数组，数组长度为 KERNEL_HEAP_SIZE
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

pub fn init_heap() {
    debug!("init_heap begin!");
    unsafe {
        // NOTE: 未知函数需要查看函数声明和源码，不要轻易相信ds输出
        HEAP_ALLOCATOR.lock().init(
            ptr::addr_of_mut!(HEAP_SPACE) as *mut u8 as usize,
            KERNEL_HEAP_SIZE,
        );
    }
    debug!("init_heap end!");
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[allow(unused)]
pub fn heap_test() {
    use alloc::vec::Vec;
    extern "C" {
        fn sbss();
        fn ebss();
    }
    let mut v: Vec<usize> = Vec::new();
    for i in 0..500 {
        v.push(i);
    }
    for (i, item) in v.iter().enumerate().take(500) {
        assert_eq!(v[i], i);
    }
    // assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    info!("heap_test passed!");
}

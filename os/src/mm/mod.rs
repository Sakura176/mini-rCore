use heap_allocator::heap_test;
use log::debug;

mod heap_allocator;

pub fn init() {
    debug!("mm init!");
    heap_allocator::init_heap();
    debug!("mm init end!");
}

pub fn test() {
    debug!("heap_allocator test begin!");
    heap_test();
    debug!("heap_allocator test end!");
}

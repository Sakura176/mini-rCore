use address::PhysAddr;
use heap_allocator::heap_test;
use log::debug;

mod address;
mod heap_allocator;

pub fn init() {
    debug!("mm init!");
    heap_allocator::init_heap();
    debug!("mm init end!");
}

pub fn test_heap() {
    debug!("heap_allocator test begin!");
    heap_test();
    debug!("heap_allocator test end!");
}

pub fn test_address() {
    debug!("test_address begin!");
    address_test();
    debug!("test_address end!");
}

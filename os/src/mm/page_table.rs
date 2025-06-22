// os/src/mm/page_table.rs

use bitflags::*;

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;  // 该值为1时，页表项合法
        const R = 1 << 1;  // 控制对应虚拟页面是否允许读
        const W = 1 << 2;  // 控制对应页面是否允许写
        const X = 1 << 3;  // 控制对应页面石佛偶允许执行
        const U = 1 << 4;  // 控制对应页面在CPU处于U特权级时是否被允许访问
        const G = 1 << 5;
        const A = 1 << 6;  // 记录上次清零后，页表项对应的虚拟页面是否被访问过
        const D = 1 << 7;  // 上次清零后，页表项对应的虚拟页面是否被修改过
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhyPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }

    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }

    pub fn ppn(&self) -> PhyPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
}

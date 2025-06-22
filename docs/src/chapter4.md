# 第四章：地址空间

> - **实验日期**: 2025-06-12
> - **代码版本**: rCore-Tutorial v3 [d7f2a1b](https://github.com/rcore-os/rCore-Tutorial-v3/commit/d7f2a1b)  
> - **QEMU版本**: 10.0.0   
> - **实验环境**: Arch Linux, Rust nightly-2024-03-05

## 目录
- [1.1 核心目标](#11-核心目标)
- [1.2 总体结构](#12-总体结构)
- [1.3 代码实现](#13-代码实现)
- [1.4 实验验证](#14-实验验证)
- [1.5 问题排查](#15-问题排查)
- [1.6 知识拓展](#16-知识拓展)

---

<a id="11-核心目标"></a>
## 1.1 核心目标
1. 内核增加连续内存分配功能
2. 实现物理页帧
3. 实现页表
4. 实现分页机制
---

<a id="12-总体结构"></a>
## 1.2 总体结构
```mermaid


```



<a id="13-代码实现"></a>
## 1.3 代码实现
1. 配置改动
    ```toml
    [package]
    name = "os"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    log = "0.4"
    sbi-rt = { version = "0.0.3", features = ["legacy"] }
    lazy_static = { version = "1.4.0", features = ["spin_no_std"] }
    riscv = { git = "https://github.com/rcore-os/riscv", features = ["inline-asm"] }
    buddy_system_allocator = "0.11.0" # 实验时使用版本，高于文档中版本，需要一定修改

    [profile.release]
    debug = true
    ```
2. 动态内存分配代码
    ```Rust
    // os/src/main.rs
    #![no_main]
    #![no_std]
    #![feature(alloc_error_handler)]

    extern crate alloc;

    // os/src/mm/heap_allocator.rs
    // TODO: 实验使用版本需添加泛型参数，该参数实际意义暂未弄懂
    #[global_allocator]
    static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

    pub fn init_heap() {
        debug!("init_heap begin!");
        unsafe {
            // NOTE: 未知函数需要查看函数声明和源码，不要轻易相信ds输出
            HEAP_ALLOCATOR.lock().init(
                // 新版本rust按文档写法无法通过编译，需按下述方法获取裸指针
                ptr::addr_of_mut!(HEAP_SPACE) as *mut u8 as usize,
                KERNEL_HEAP_SIZE,
            );
        }
        debug!("init_heap end!");
    }

    ```
3. SBI调用封装
    封装`panic!`宏，`panic!`宏在Rust标准库中有具体实现，用于在程序出错时打印出错位置和原因并杀死当前应用。移除标准库后需要实现简易版本来通过测试
    ```Rust
    // os/src/lang_item.rs
    use core::panic::PanicIfoo;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
    ```


4. 基本输出实现

<a id="14-实验验证"></a>
## 1.4 实验验证

### 测试代码  
1. 动态内存分配测试代码  

    ```Rust
    pub struct Heap<const ORDER: usize> {
        // buddy system with max order of `ORDER`
        free_list: [linked_list::LinkedList; ORDER],

        // statistics
        user: usize,
        allocated: usize,
        total: usize,
    }
    ```
2. 地址也页表转换测试代码

    ```Rust
        
    ```
### 输出
## 1.5 问题排查
## 1.6 知识拓展

## 本章总结

|关键概念|实现要点|代码位置|
|------|-------|------|
|裸机环境|移除标准库依赖|Cargo.toml|
|内核入口|_start函数定义|src/main.rs|
|基本输出|SBI控制台封装|src/sbi.rs|
|内存初始化|.bss段清零操作|src/main.rs|
|系统关机|SBI_SHUTDOWN调用|src/sbi.rs|

## 下一步计划:
+ [x] 第二章: 批处理系统实现
+ [x] 添加应用加载功能
+ [x] 实现特权级切换机制

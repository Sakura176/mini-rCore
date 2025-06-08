# 设置内核执行环境
# 操作系统内核入口点的汇编代码
# 主要负责初始化栈指针并跳转到Rust主函数

### 定义代码段（可执行代码部分）
    .section .text.entry
    .globl _start
_start:
    # 栈顶指针存入sp寄存器
    la sp, boot_stack_top
    call rust_main

### 定义未初始化的栈内存空间
    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:

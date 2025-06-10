# 第一章：启动流程与裸机环境构建

> - **实验日期**: 2025-06-09
> - **代码版本**: rCore-Tutorial v3 [d7f2a1b](https://github.com/rcore-os/rCore-Tutorial-v3/commit/d7f2a1b)  
> - **QEMU版本**: 10.0.0   
> - **实验环境**: Arch Linux, Rust nightly-2024-03-05

## 目录
- [1.1 核心目标](#11-核心目标)
- [1.2 关键步骤](#12-关键步骤)
- [1.3 代码实现](#13-代码实现)
- [1.4 实验验证](#14-实验验证)
- [1.5 问题排查](#15-问题排查)
- [1.6 知识拓展](#16-知识拓展)

---

<a id="11-核心目标"></a>
## 1.1 核心目标
1. 建立裸机运行环境
2. 移除标准库依赖
3. 实现基本输出功能
4. 构建最小化内核入口
5. 实现系统关机功能

---

<a id="12-关键步骤"></a>
## 1.2 关键步骤

### 启动流程概览
```mermaid
sequenceDiagram
    participant BIOS
    participant Bootloader
    participant Kernel
    participant SBI
    
    BIOS->>Bootloader: 加载并执行
    Bootloader->>SBI: 初始化硬件
    SBI->>Kernel: 移交控制权
    Kernel->>Kernel: 初始化基本环境
    Kernel->>SBI: 关机请求
    SBI->>Hardware: 系统关机
```

#### 开发步骤

1. 创建裸机项目结构
2. 配置no_std环境
3. 实现panic处理程序
4. 构建内核入口点
5. 实现SBI封装调用
6. 添加基本输出功能

<a id="13-代码实现"></a>
## 1.3 代码实现
1. 项目初始化
```Bash
# 创建项目
cargo new mini-rCore
cd mini-rCore

# 配置Cargo.toml
[package]
name = "mini-rCore"
version = "0.1.0"
edition = "2021"

[dependencies]
riscv = { git = "https://github.com/rcore-os/riscv", rev = "340e5bf" }
```
2. 裸机入口配置
3. SBI调用封装
4. 基本输出实现

<a id="14-实验验证"></a>
## 1.4 实验验证
### 测试代码
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

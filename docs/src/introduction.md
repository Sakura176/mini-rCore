# rCore学习文档

> **环境信息**  
> - 教程版本：[v3.6.0](https://github.com/rcore-os/rCore-Tutorial-Book-v3)  
> - 代码仓库：[rCore-Tutorial-v3](https://github.com/rcore-os/rCore-Tutorial-v3)  
> - Rust 工具链：`rustc 1.85.0-nightly (426d17342 2024-12-21)`
> - 操作系统：`Arch Linux`
> - 模拟器：`qemu-10.0.0`

## 目录
- [1. 环境配置](#1-环境配置)
- [2. 章节学习记录](#2-章节学习记录)
- [3. 关键机制图解](#3-关键机制图解)
- [4. Rust 专题笔记](#4-rust-专题笔记)
- [5. 调试日志](#5-调试日志)
- [6. 扩展实验](#6-扩展实验)
- [7. 资源索引](#7-资源索引)

---

<a id="1-环境配置"></a>
## 1. 环境配置

### 工具链安装
```bash
# 指定版本工具链
rustup toolchain install nightly-2024-03-05
rustup default nightly-2024-03-05
rustup target add riscv64gc-unknown-none-elf

# 验证安装
cargo --version
rustc --version
```
### qemu模拟器安装
Arch上安装qemu模拟器比较简单，运行下述命令即可：
```shell
sudo pacman -S qemu-system-riscv
```




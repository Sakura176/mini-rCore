qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ./bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/mini-rCore.bin,addr=0x80200000 \
    -gdb tcp::1235 -S

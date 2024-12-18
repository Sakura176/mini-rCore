qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios /home/yc/mini-rCore/bootloader/rustsbi-qemu.bin \
    -device loader,file=/home/yc/mini-rCore/os/target/riscv64gc-unknown-none-elf/release/mini-rCore,addr=0x80200000 \
    -gdb tcp::1235

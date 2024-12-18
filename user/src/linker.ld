OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x80400000;

SECTIONS
{
  . = BASE_ADDRESS;
  /* 代码区 */
  .text : {
    *(.text.entry)
    *(.text .text.*)
  }
  /* 常量区 */
  .rodata : {
    *(.rodata .rodata.*)
    *(.srodata .srodata.*)
  }
  /* 全局区（已初始化） */
  .data : {
    *(.data .data.*)
    *(.sdata .sdata.*)
  }
  /* 全局区（未初始化） */
  .bss : {
    start_bss = .;
    *(.bss .bss.*)
    *(.sbss .sbss.*)
    end_bss = .;
  }
  /DISCARD/ : {
      *(.eh_frame)
      *(.debug*)
  }
}

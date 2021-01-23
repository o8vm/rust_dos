ENTRY(_start)

MEMORY {
  dos : org = 0x100, len = (0xFFFF - 0x100)
}

SECTIONS {
  .text   : { *(.startup) *(.text .text.*) }   > dos
  .rodata : { *(.rodata .rodata.*) } > dos
  .data   : { *(.data) }   > dos
  .bss    : { *(.bss) }    > dos
  .stack  : { *(.stack) }  > dos
  /DISCARD/ : { *(.eh_frame*) }
}

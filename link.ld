ENTRY(_start)

MEMORY
{
    ROM (rx)  : ORIGIN = 0x80000000, LENGTH = 64K
    RAM (rwx) : ORIGIN = 0x80010000, LENGTH = 64K
}

SECTIONS
{
    .text : {
        KEEP(*(.text.entry))
        *(.text .text.*)
    } > ROM

    .rodata : {
        *(.rodata .rodata.*)
    } > ROM

    .data : {
        *(.data .data.*)
    } > RAM AT > ROM

    .bss : {
        _bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        _bss_end = .;
    } > RAM

    /DISCARD/ : {
        *(.comment)
        *(.note .note.*)
    }
}

ENTRY(_start)

SECTIONS
{
    /*
     * Load the kernel at 1 MiB. The first megabyte of physical memory is
     * occupied by BIOS data, legacy hardware mappings, and the real-mode
     * interrupt vector table, so 1 MiB is the conventional safe starting
     * address for a kernel loaded by a multiboot2 bootloader.
     */
    . = 1M;

    /*
     * The multiboot2 spec requires the header to appear within the first
     * 32 KiB of the kernel image, on an 8-byte boundary. Placing it first
     * in the output guarantees that regardless of how much text precedes it.
     */
    .boot ALIGN(8) :
    {
        KEEP(*(.multiboot))
    }

    .text ALIGN(4K) :
    {
        *(.text.boot)       /* Boot entry code, before long mode is set up */
        *(.text .text.*)
    }

    .rodata ALIGN(4K) : { *(.rodata .rodata.*) }
    .data   ALIGN(4K) : { *(.data   .data.*)   }

    .bss ALIGN(4K) :
    {
        *(COMMON)
        *(.bss .bss.*)
    }

    /* Drop metadata sections that the kernel doesn't need at runtime. */
    /DISCARD/ : { *(.eh_frame) *(.note .note.*) }
}

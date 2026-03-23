# boot.s — protected mode entry and long mode transition
#
# GRUB loads us here in 32-bit protected mode. Before any Rust code can run
# we need to get into 64-bit long mode. The steps are:
#
#   1. Set up page tables (long mode requires paging to be active)
#   2. Enable PAE (Physical Address Extension) in CR4
#   3. Point CR3 at our top-level page table
#   4. Set the long mode enable bit in the EFER model-specific register
#   5. Enable paging in CR0 (this actually activates long mode)
#   6. Load a 64-bit GDT
#   7. Far jump into the 64-bit code segment
#   8. Reload segment registers and call kernel_main

.code32
.section .text.boot, "ax"
.global _start

_start:
    cli
    mov esp, OFFSET __boot_stack_top

    # Save the two values GRUB hands us. Both edi and esi are callee-saved in
    # the 32-bit ABI, so they survive through setup_page_tables and the rest
    # of the transition.
    #
    #   edi = multiboot2 info address (from ebx) → kernel_main arg 1 (rdi)
    #   esi = bootloader magic        (from eax) → kernel_main arg 2 (rsi)
    mov edi, ebx
    mov esi, eax

    call setup_page_tables

    # Step 2: enable PAE. Long mode cannot be activated without it.
    mov eax, cr4
    or eax, (1 << 5)           # CR4.PAE
    mov cr4, eax

    # Step 3: point CR3 at our PML4. The CPU walks this table on every
    # memory access once paging is on.
    mov eax, OFFSET __pml4
    mov cr3, eax

    # Step 4: set LME (long mode enable) in the EFER MSR.
    # EFER is MSR 0xC0000080. rdmsr/wrmsr read and write MSRs using ecx
    # as the register number and edx:eax as the 64-bit value.
    mov ecx, 0xC0000080
    rdmsr
    or eax, (1 << 8)           # EFER.LME
    wrmsr

    # Step 5: enable paging. Setting CR0.PG while LME is set activates
    # long mode (the CPU is now in the "compatibility" sub-mode until we
    # perform a far jump into a 64-bit code segment).
    mov eax, cr0
    or eax, (1 << 31)          # CR0.PG
    mov cr0, eax

    # Step 6: load a GDT that has a 64-bit code segment descriptor.
    lgdt [__gdt_descriptor]

    # Step 7: far jump to flush the instruction pipeline and fully enter
    # 64-bit mode. The selector 0x08 is the offset of our code descriptor
    # in the GDT (past the mandatory null descriptor).
    #
    # LLVM's Intel-syntax assembler doesn't support direct far jumps to
    # immediate addresses, so we use the push/retf trick: set up the stack
    # as if we're about to return from a far call, then retf.
    push 0x08
    mov eax, OFFSET __long_mode_entry
    push eax
    retf


# Sets up three levels of page tables that identity-map the first 1 GiB
# of physical memory using 2 MiB pages.
#
# x86_64 uses a four-level page table hierarchy:
#
#   CR3 → PML4 → PDPT → PD → PT → physical page
#
# With 2 MiB "huge" pages we skip the last level (PT). One PD covers
# 512 × 2 MiB = 1 GiB, which is enough to reach our kernel at 1 MiB
# and the VGA buffer at 0xB8000.
setup_page_tables:
    # PML4[0] → PDPT  (flags: present + writable)
    mov eax, OFFSET __pdpt
    or eax, 0b11
    mov [__pml4], eax

    # PDPT[0] → PD
    mov eax, OFFSET __pd
    or eax, 0b11
    mov [__pdpt], eax

    # PD[i] → 2 MiB page at physical address i * 2 MiB
    # Flags 0b10000011: present + writable + huge page (PS bit)
    mov ecx, 0
1:
    mov eax, ecx
    shl eax, 21                # i * 2 MiB  (2 MiB = 1 << 21)
    or eax, 0b10000011
    mov [__pd + ecx * 8], eax

    inc ecx
    cmp ecx, 512
    jne 1b

    ret


# --- 64-bit code -------------------------------------------------------------

.code64
__long_mode_entry:
    # Reload the data segment registers. In 64-bit mode the CPU mostly
    # ignores segment bases and limits, but the registers should point at a
    # valid data descriptor (0x10) or be zeroed.
    mov ax, 0x10
    mov ss, ax
    mov ds, ax
    mov es, ax
    xor ax, ax                 # fs/gs are reserved for special use; zero them
    mov fs, ax
    mov gs, ax

    # Writing to a 32-bit register in 64-bit mode zero-extends to the full
    # 64-bit register. Do this explicitly so kernel_main receives clean u64
    # values regardless of what was in the upper 32 bits before long mode.
    mov edi, edi               # zero-extend info address → rdi (arg 1)
    mov esi, esi               # zero-extend magic        → rsi (arg 2)
    call kernel_main

    # kernel_main is diverging and should never return. Halt defensively.
2:
    hlt
    jmp 2b


# --- Global Descriptor Table -------------------------------------------------
#
# The GDT must contain a 64-bit code segment with the L (long mode) bit set.
# In 64-bit mode the CPU ignores most segment fields (base, limit, most flags),
# but the L bit and the present bit still matter for the code segment.
#
# Each descriptor is 8 bytes. Encoding for the code segment 0x00AF9A000000FFFF:
#   Byte 7: Base[31:24]      = 0x00
#   Byte 6: G=1 D=0 L=1 A=0 Limit[19:16]=0xF  → 0xAF
#   Byte 5: P=1 DPL=0 S=1 Type=1010 (exec/read) → 0x9A
#   Byte 4: Base[23:16]      = 0x00
#   Bytes 2-3: Base[15:0]    = 0x0000
#   Bytes 0-1: Limit[15:0]   = 0xFFFF

.section .rodata
.align 8
__gdt:
    .quad 0x0000000000000000   # 0x00  null descriptor (required)
    .quad 0x00AF9A000000FFFF   # 0x08  64-bit kernel code: P=1, L=1, DPL=0
    .quad 0x00CF92000000FFFF   # 0x10  64-bit kernel data: P=1, DPL=0
__gdt_end:

.align 4
__gdt_descriptor:
    .word __gdt_end - __gdt - 1   # limit = size - 1
    .long __gdt                    # base  (32-bit address, loaded before paging)


# --- BSS ---------------------------------------------------------------------

.section .bss

# Page tables must be 4 KiB aligned. Placing them in .bss means they are
# zero-initialised by the bootloader, so we only need to write the entries
# we actually use.
.align 4096
__pml4: .space 4096
__pdpt: .space 4096
__pd:   .space 4096

# Boot stack (16 KiB)
.align 16
__boot_stack_bottom: .space 16384
__boot_stack_top:

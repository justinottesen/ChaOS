;
; Prepares the system for kernel execution and kicks it off
;
; This involves a lot more steps than the first stage:
;   1. Enable A20 - This is needed to access memory above 1MB. It is typically done by default on
;       modern systems, older systems had it disabled for legacy compatability.
;   2. Read the BIOS memory map - The BIOS tells us information about our system's memory. We will
;       need to hand this off to the kernel.
;   3. Load the kernel from disk - Similarly to how stage 1 loaded stage 2 from disk, stage 2 must
;       load the kernel.
;   4. Load the Global Descriptor Table - This tells the CPU how we are using different segments of
;       memory, and is necessary for the next step.
;   5. Jump to 32-bit protected mode - Pretty self explanatory
;   6. Set up page tables - This is how the CPU resolves virtual addresses and is needed for 64-bit
;       addressing.
;   7. Jump to 64-bit long mode - Unlocks the full address space.
;   8. Parse the kernel ELF and jump to it - Finally hands control over to the kernel.
;

;
; Stage 1 hands us control in 16-bit real mode at STAGE2_LOAD_ADDR
;
[bits 16]
[org STAGE2_LOAD_ADDR]

%include "lib/print_macros.inc"

;
; Our entry point, jumped to from stage 1
;
stage2_16:
    ;
    ; Stage 1 passes us the drive number on the stack
    ;
    pop dx
    mov [drive_number], dl

    ;
    ; Enable the A20 address line. This gives us access to slightly more memory now, but it is
    ; necessary for 
    ;
    call enable_a20
    jz .a20_enabled

    ;
    ; If we failed to enable A20, we cannot proceed
    ;
    PRINT err_failed_a20
    jmp .hang

.a20_enabled:
    ;
    ; Query the BIOS for the memory map
    ;
    call detect_memory
    jnc .mem_map_loaded

    ;
    ; Can't continue if we fail to load the memory map
    ;
    PRINT err_failed_mmap
    jmp .hang

.mem_map_loaded:

.hang:
    cli
    hlt
    jmp $

drive_number:
    db 0

%include "lib/print.inc"
%include "lib/a20.inc"
%include "lib/mmap.inc"

;
; Strings
;
DEFSTRING msg_stage2, "Hello from stage 2!", NEWLINE
DEFSTRING err_failed_a20, "BOOT ERROR: Failed to enable A20", NEWLINE
DEFSTRING err_failed_mmap, "BOOT ERROR: Failed to read the memory map", NEWLINE
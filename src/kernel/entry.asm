;
; Kernel entry point
; Sets up the stack and calls into the Rust kernel.
;

[bits 32]

global kernel_entry
extern kernel_main

section .text.kernel_entry
kernel_entry:
    mov esp, 0x90000    ; stack grows downward from here (safe conventional memory)
    cld                 ; clear direction flag (required by C/Rust calling convention)
    call kernel_main
.hang:
    hlt
    jmp .hang

;
; Prepares the system for kernel execution and kicks it off
;

;
; Stage 1 hands us control in 16-bit real mode at STAGE2_LOAD_ADDR
;
[bits 16]
[org STAGE2_LOAD_ADDR]

;
; Defines some useful macros in defining strings for printing
;
%include "lib/print_macros.inc"

;
; Our entry point, jumped to from stage 1
;
start:
    PRINT msg_stage2
;
; This disables interrupts and infinitely loops the CPU. In a real bootloader, this will not be
; reached
;
.hang:
    cli
    hlt
    jmp $

;
; Imported functionality from lib files
;
%include "lib/print.inc"

;
; Strings
;
DEFSTRING msg_stage2, "Hello from stage 2!", NEWLINE
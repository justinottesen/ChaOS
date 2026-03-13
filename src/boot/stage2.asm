;
; Stage 2 bootloader
;

[bits 16]
[org 0x7E00]

start:
    mov si, hello_string
    mov cx, hello_length
    call print_string

    jmp $

%include "lib/print.asm"

;
; Program Data
;

hello_string: db "Hello from stage 2!"
hello_length: equ $ - hello_string

times 512 - ($ - $$) db 0      ; Pad to a full sector

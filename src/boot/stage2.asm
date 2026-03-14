;
; Stage 2 bootloader
;

[bits 16]
[org 0x7E00]

start:
    mov [boot_drive], dl        ; DL preserved from stage 1

    mov si, msg_stage2
    mov cx, msg_stage2_len
    call print_string

    mov si, msg_enabling_a20
    mov cx, msg_enabling_a20_len
    call print_string

    call enable_a20

    mov si, msg_a20_enabled
    mov cx, msg_a20_enabled_len
    call print_string

    mov si, msg_gdt
    mov cx, msg_gdt_len
    call print_string

    call load_gdt

    mov si, msg_gdt_loaded
    mov cx, msg_gdt_loaded_len
    call print_string

    mov si, msg_loading_kernel
    mov cx, msg_loading_kernel_len
    call print_string

    ; Load kernel into memory at 0x10000 (ES=0x1000, BX=0)
    mov ax, 0x1000
    mov es, ax
    mov ax, 3                   ; LBA sector 3 (stage2 occupies sectors 1-2)
    mov cl, 1                   ; 1 sector
    mov bx, 0                   ; ES:BX = 0x1000:0 = physical 0x10000
    call disk_read
    xor ax, ax
    mov es, ax                  ; restore ES = 0

    mov si, msg_jumping_kernel
    mov cx, msg_jumping_kernel_len
    call print_string

    jmp switch_to_protected_mode

%include "lib/gdt.asm"
%include "lib/a20.asm"
%include "lib/disk.asm"
%include "lib/print.asm"
%include "lib/protected_mode.asm"

;
; Program Data
;

boot_drive: db 0

msg_stage2:             db "Stage 2 loaded.", 0x0D, 0x0A
msg_stage2_len:         equ $ - msg_stage2

msg_enabling_a20:       db "Enabling A20...", 0x0D, 0x0A
msg_enabling_a20_len:   equ $ - msg_enabling_a20

msg_a20_enabled:        db "A20 enabled.", 0x0D, 0x0A
msg_a20_enabled_len:    equ $ - msg_a20_enabled

msg_gdt:                db "Setting up GDT...", 0x0D, 0x0A
msg_gdt_len:            equ $ - msg_gdt

msg_gdt_loaded:         db "GDT loaded.", 0x0D, 0x0A
msg_gdt_loaded_len:     equ $ - msg_gdt_loaded

msg_loading_kernel:     db "Loading kernel...", 0x0D, 0x0A
msg_loading_kernel_len: equ $ - msg_loading_kernel

msg_jumping_kernel:     db "Jumping to kernel...", 0x0D, 0x0A
msg_jumping_kernel_len: equ $ - msg_jumping_kernel

times 1024 - ($ - $$) db 0     ; Pad to 2 sectors

;
; Loads stage 2 into 0x7E00 and jumps to it
;

[org 0x7C00]                    ; The BIOS loads us at this address

; Entry point
start:
    ; The BIOS may leave segment registers in a weird state
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov [boot_drive], dl        ; BIOS stores the boot drive number in dl

    mov si, msg_booting
    mov cx, msg_booting_len
    call print_string

    mov si, msg_loading_stage2
    mov cx, msg_loading_stage2_len
    call print_string

    mov ax, 1                   ; LBA sector 1
    mov cl, 2                   ; read 2 sectors
    mov bx, 0x7E00              ; destination address
    call disk_read

    mov si, msg_jumping_stage2
    mov cx, msg_jumping_stage2_len
    call print_string

    jmp 0x7E00                  ; Jump to stage 2

%include "lib/disk.asm"
%include "lib/print.asm"

;
; Program Data
;

boot_drive: db 0

msg_booting:            db "Booting...", 0x0D, 0x0A
msg_booting_len:        equ $ - msg_booting

msg_loading_stage2:     db "Loading stage 2...", 0x0D, 0x0A
msg_loading_stage2_len: equ $ - msg_loading_stage2

msg_jumping_stage2:     db "Jumping to stage 2...", 0x0D, 0x0A
msg_jumping_stage2_len: equ $ - msg_jumping_stage2

;
; Padding and magic BIOS number
;

times 510 - ($ - $$) db 0       ; Pad the rest of the boot sector with 0s

dw 0xAA55                       ; Write the magic number so BIOS can find us

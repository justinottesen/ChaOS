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

    mov ax, 1                   ; LBA sector 1
    mov cl, 1                   ; read 1 sector
    mov bx, 0x7E00              ; destination address
    call disk_read

    jmp 0x7E00                  ; Jump to stage 2

%include "lib/disk.asm"

;
; Program Data
;

boot_drive: db 0

;
; Padding and magic BIOS number
;

times 510 - ($ - $$) db 0       ; Pad the rest of the boot sector with 0s

dw 0xAA55                       ; Write the magic number so BIOS can find us

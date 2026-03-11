;
; Prints hello, then loops forever
;

mov ah, 0x0E                    ; Set teletype mode

mov al, 'H'                     ; Load a char
int 0x10                        ; Call bios interrupt to print
mov al, 'e'                     ; Repeat for remaining letters
int 0x10
mov al, 'l'
int 0x10
mov al, 'l'
int 0x10
mov al, 'o'
int 0x10

jmp $                           ; Jump to current address

;
; Padding and magic BIOS number
;

times 510 - ($ - $$) db 0       ; Pad the rest of the boot sector with 0s

dw 0xAA55                       ; Write the magic number so BIOS can find us

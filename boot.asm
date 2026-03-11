;
; Prints hello, then loops forever
;

[org 0x7C00]                    ; The BIOS loads us at this address

; Entry point
start:
    ; The BIOS may leave segment registers in a weird state
    xor ax, ax
    mov ds, ax
    mov es, ax

    mov si, HELLO_WORLD_STRING      ; Print the hello world string
    call print_string

    jmp $                           ; Jump to current address

;
; Prints a string to the screen
;
; Params:
;   - ds:si points to the string
;
print_string:
    push si
    push ax
    push bx

    mov ah, 0x0E                ; Set teletype mode
.loop:
    lodsb                       ; Loads the next character into al
    or al, al                   ; Check if we are at null terminator
    jz .done                    ; Jump to done if so

    int 0x10                    ; Interrupt to print a char

    jmp .loop

.done:
    pop bx
    pop ax
    pop si
    ret

;
; Program Data
;

HELLO_WORLD_STRING:
    db "Hello, world!", 0       ; Null-terminated so we know when the string ends

;
; Padding and magic BIOS number
;

times 510 - ($ - $$) db 0       ; Pad the rest of the boot sector with 0s

dw 0xAA55                       ; Write the magic number so BIOS can find us

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

    mov si, hello_world_string
    mov cx, hello_world_length
    call print_string

    jmp $                           ; Jump to current address

;
; Prints a string to the screen
;
; Params:
;   - ds:si = address of the string
;   - cx    = length of the string
;
print_string:
    push si

    mov ah, 0x0E                ; Set teletype mode
.loop:
    jcxz .done                  ; Check if length counter is 0
    lodsb                       ; Loads the next character into al
    int 0x10                    ; Interrupt to print a char
    dec cx                      ; Decrements length counter in cx
    jmp .loop

.done:
    pop si
    ret

;
; Program Data
;

hello_world_string: db "Hello, world!"
hello_world_length: equ $ - hello_world_string

;
; Padding and magic BIOS number
;

times 510 - ($ - $$) db 0       ; Pad the rest of the boot sector with 0s

dw 0xAA55                       ; Write the magic number so BIOS can find us

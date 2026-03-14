[bits 16]

;
; Prints a message and halts forever
;
; Params:
;   ds:si = address of the message
;   cx    = length of the message
;
fatal_error:
    call print_string
    jmp $

;
; Prints a string to the screen
;
; Params:
;   ds:si = address of the string
;   cx    = length of the string
;
print_string:
    push si

    mov ah, 0x0E                ; Set teletype mode
.loop:
    jcxz .done
    lodsb
    int 0x10
    dec cx
    jmp .loop

.done:
    pop si
    ret

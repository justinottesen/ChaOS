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

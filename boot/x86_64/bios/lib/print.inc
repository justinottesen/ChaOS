;
; Provides print function
;

%ifndef __PRINT_INC
%define __PRINT_INC

%include "lib/print_macros.inc"

;
; print a string to the screen
;
; Params:
;   ds:si = address of the string
;   cx    = length of the string
;
print_string:
    jcxz .ret       ; Skip if 0 length provided

    push si         ; Preserve si on stack

    mov ah, 0x0E    ; Set the teletype mode
.loop:
    lodsb           ; al = (*si)++
    int 0x10        ; Interrupt to print char
    loop .loop      ; jump if (--cx) != 0

    pop si          ; Restore si from stack
.ret:
    ret

%endif
;
; A simple boot sector program that loops forever
;

; Loop forever
loop:
    jmp loop

times 510 - ($ - $$) db 0

dw 0xAA55

;
; Stage 2 placeholder - just a string for testing the disk read
;

dw msg_end - msg_start          ; 2-byte length prefix
msg_start: db "Hello, world!"
msg_end:

times 512 - ($ - $$) db 0      ; Pad to a full sector

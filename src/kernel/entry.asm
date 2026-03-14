;
; Kernel entry point
;

[bits 32]
[org 0x10000]

kernel_entry:
    ; Write "OK" in bright green at row 0, col 0 (most visible position)
    ; to confirm the kernel is actually executing.
    mov byte [0xB8000], 'O'
    mov byte [0xB8001], 0x0A   ; green on black
    mov byte [0xB8002], 'K'
    mov byte [0xB8003], 0x0A

.hang:
    hlt
    jmp .hang

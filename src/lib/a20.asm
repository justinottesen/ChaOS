[bits 16]

;
; Enables the A20 address line, allowing access to memory above 1MB.
; Tries the BIOS method first, then falls back to Fast A20 via port 0x92.
; Halts if neither method works.
;
enable_a20:
    mov ax, 0x2401              ; BIOS: enable A20
    int 0x15

    call a20_enabled
    jnz .done

    ; Fast A20 via port 0x92
    in al, 0x92
    or al, 0x02                 ; set A20 bit
    and al, 0xFE                ; clear bit 0 - setting it triggers a system reset
    out 0x92, al

    call a20_enabled
    jnz .done

.error:
    mov si, .msg_error
    mov cx, .msg_error_len
    call fatal_error

.msg_error:     db "Failed to enable A20.", 0x0D, 0x0A
.msg_error_len: equ $ - .msg_error

.done:
    ret

;
; Checks whether A20 is enabled.
;
; Writes different values to 0x0000:0x0500 and 0xFFFF:0x0510, which map to
; physical addresses 0x000500 and 0x100500. If A20 is off, bit 20 is forced to
; 0 and both addresses wrap to 0x000500 - the writes alias and the values match.
;
; Returns: ZF=0 (jnz taken) if enabled, ZF=1 (jz taken) if disabled
;
a20_enabled:
    push ds
    push es
    push di
    push si

    xor ax, ax
    mov ds, ax
    mov di, 0x0500              ; ds:di = physical 0x000500

    mov ax, 0xFFFF
    mov es, ax
    mov si, 0x0510              ; es:si = physical 0x100500

    mov al, [ds:di]             ; save originals
    push ax
    mov al, [es:si]
    push ax

    mov byte [ds:di], 0x00
    mov byte [es:si], 0xFF

    cmp byte [ds:di], 0xFF      ; if A20 is off, the write to es:si wrapped and
                                ; overwrote ds:di, so this comparison is equal

    pop ax                      ; restore originals
    mov [es:si], al
    pop ax
    mov [ds:di], al

    pop si
    pop di
    pop es
    pop ds
    ret

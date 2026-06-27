;
; Enables the A20 line
;

%ifndef __A20_INC
%define __A20_INC

;
; Checks if A20 is enabled by writing to 0xFFFF:0x510 and seeing if it gets wrapped around and
; instead updates 0x0000:0x0500
;
; Resturns:
;   Zero flag: set if A20 is enabled, clear if disabled
;
check_a20:
    push es
    push ds

    xor ax, ax
    mov ds, ax      ; ds = 0x0000
    not ax
    mov es, ax      ; es = 0xFFFF

    mov byte [ds:0x0500], 0x00
    mov byte [es:0x0510], 0xFF      ; wraps to 0x0500 if A20 off
    cmp byte [ds:0x0500], 0x00      ; ZF set if wrapped (A20 off)

    pop ds
    pop es
    ret

;
; Enables the A20 address line
;
; Returns:
;   Zero flag: set if A20 could not be enabled, clear if enabled
;
enable_a20:
    ;
    ; Check if it was already enabled - Most modern BIOSes enable by default
    ;
    call check_a20
    jz .done

    ;
    ; Try using the BIOS function to enable it
    ;
    mov ax, 0x2401
    int 0x15
    call check_a20
    jz .done

    ;
    ; Try using the Fast A20 gate
    ;
    in al, 0x92
    or al, 0x02
    out 0x92, al
    call check_a20
    jz .done

.done:
    ret

%endif

[bits 16]

;
; Loads the GDT
;
load_gdt:
    lgdt [gdt_descriptor]
    ret

;
; Global Descriptor Table
;
gdt_start:

gdt_null:                           ; null descriptor - required, must be first
    dq 0

gdt_code:                           ; code segment descriptor
    dw 0xFFFF                       ; limit 15:0
    dw 0x0000                       ; base 15:0
    db 0x00                         ; base 23:16
    db 10011010b                    ; access: present, ring 0, code segment, executable, readable
    db 11001111b                    ; flags: 4KB granularity, 32-bit | limit 19:16
    db 0x00                         ; base 31:24

gdt_data:                           ; data segment descriptor
    dw 0xFFFF                       ; limit 15:0
    dw 0x0000                       ; base 15:0
    db 0x00                         ; base 23:16
    db 10010010b                    ; access: present, ring 0, data segment, writable
    db 11001111b                    ; flags: 4KB granularity, 32-bit | limit 19:16
    db 0x00                         ; base 31:24

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1     ; size of GDT minus 1
    dd gdt_start                    ; linear address of GDT

; Segment selectors - index into the GDT, used after the mode switch
CODE_SEG equ gdt_code - gdt_start  ; 0x08
DATA_SEG equ gdt_data - gdt_start  ; 0x10

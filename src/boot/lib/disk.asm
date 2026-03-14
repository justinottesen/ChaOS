[bits 16]

;
; Reads sectors from disk
;
; Params:
;   ax = LBA sector number
;   cl = number of sectors to read
;   bx = destination address (in segment es)
;
; Depends on:
;   boot_drive = label of a byte holding the BIOS drive number
;
disk_read:
    push ax
    push bx
    push cx
    push dx

    mov [.sector_count], cl
    mov [.lba], ax
    mov byte [.retries], 3

.attempt:
    ; Convert LBA to CHS
    ; sector   = (LBA % sectors_per_track) + 1
    ; head     = (LBA / sectors_per_track) % heads_per_cylinder
    ; cylinder = (LBA / sectors_per_track) / heads_per_cylinder
    mov ax, [.lba]
    xor dx, dx
    mov cx, 18                  ; sectors per track (floppy geometry)
    div cx                      ; ax = LBA / 18, dx = LBA % 18
    inc dx
    mov [.sector], dl           ; save sector number (1-based) before CX is clobbered

    xor dx, dx
    mov cx, 2                   ; heads per cylinder
    div cx                      ; ax = cylinder, dx = head
    mov ch, al                  ; ch = cylinder
    mov dh, dl                  ; dh = head
    mov cl, [.sector]           ; cl = sector number (1-based)

    mov ah, 0x02
    mov al, [.sector_count]
    mov dl, [boot_drive]
    int 0x13
    jnc .done

    dec byte [.retries]
    jz .error

    ; Reset drive before retrying
    xor ax, ax
    mov dl, [boot_drive]
    int 0x13

    jmp .attempt

.error:
    mov si, .msg_error
    mov cx, .msg_error_len
    call fatal_error

.done:
    pop dx
    pop cx
    pop bx
    pop ax
    ret

.lba:           dw 0
.sector_count:  db 0
.retries:       db 0
.sector:        db 0
.msg_error:     db "Disk read failed.", 0x0D, 0x0A
.msg_error_len: equ $ - .msg_error

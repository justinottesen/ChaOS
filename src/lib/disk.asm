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

    ; Convert LBA to CHS
    ; sector   = (LBA % sectors_per_track) + 1
    ; head     = (LBA / sectors_per_track) % heads_per_cylinder
    ; cylinder = (LBA / sectors_per_track) / heads_per_cylinder
    xor dx, dx
    mov cx, 18                  ; sectors per track (floppy geometry)
    div cx                      ; ax = LBA / 18, dx = LBA % 18
    inc dx
    mov cl, dl                  ; cl = sector number (1-based)

    xor dx, dx
    mov cx, 2                   ; heads per cylinder
    div cx                      ; ax = cylinder, dx = head
    mov ch, al                  ; ch = cylinder
    mov dh, dl                  ; dh = head

    mov ah, 0x02
    mov al, [.sector_count]
    mov dl, [boot_drive]
    int 0x13
    jc .error

    pop dx
    pop cx
    pop bx
    pop ax
    ret

.error:
    hlt

.sector_count: db 0

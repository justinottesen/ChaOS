;
; Loads sector 1 into 0x7E00 and prints the string stored there
;

[org 0x7C00]                    ; The BIOS loads us at this address

; Entry point
start:
    ; The BIOS may leave segment registers in a weird state
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov [boot_drive], dl        ; BIOS stores the boot drive number in dl

    mov ax, 1                   ; LBA sector 1
    mov cl, 1                   ; read 1 sector
    mov bx, 0x7E00              ; destination address
    call disk_read

    mov cx, [0x7E00]            ; first 2 bytes are the string length
    mov si, 0x7E02              ; string data follows
    call print_string

    jmp $                       ; Jump to current address

;
; Reads sectors from disk
;
; Params:
;   ax = LBA sector number
;   cl = number of sectors to read
;   bx = destination address (in segment es)
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
    jcxz .done                  ; Check if length counter is 0
    lodsb                       ; Loads the next character into al
    int 0x10                    ; Interrupt to print a char
    dec cx                      ; Decrements length counter in cx
    jmp .loop

.done:
    pop si
    ret

;
; Program Data
;

boot_drive: db 0

;
; Padding and magic BIOS number
;

times 510 - ($ - $$) db 0       ; Pad the rest of the boot sector with 0s

dw 0xAA55                       ; Write the magic number so BIOS can find us

;
; Disk read functions (EDD with CHS fallback)
;

%ifndef __DISK_INC
%define __DISK_INC

;
; Caches the result of whether EDD is supported
;
EDD_UNINIT      equ 0
EDD_UNSUPPORTED equ 1
EDD_SUPPORTED   equ 2
edd_supported:
    db EDD_UNINIT

;
; Checks whether EDD is supported (otherwise we have to use CHS addressing)
;
; Params:
;   dl = drive number
;
; Returns:
;   Sets the `edd_supported` label with either `EDD_UNSUPPORTED` or `EDD_SUPPORTED`
;
check_edd:
    push ax
    push bx
    push dx

    mov ah, 0x41
    mov bx, 0x55AA
    ; dl already holds drive number
    int 0x13
    jc .not_supported
    cmp bx, 0xAA55
    jne .not_supported
.supported:
    mov byte [edd_supported], EDD_SUPPORTED
    jmp .ret
.not_supported:
    mov byte [edd_supported], EDD_UNSUPPORTED
.ret:
    pop dx
    pop bx
    pop ax
    ret

;
; Read sectors from disk using the DAP (Disk Address Packet) as the interface.
; Tries EDD first, falls back to CHS if EDD is not supported.
;
; Params:
;   si = pointer to the DAP
;   dl = drive number
;
; Returns:
;   Carry flag: set on failure, clear on success
;
read_disk:
    mov [.drive_number], dl

    push ax
    push bx
    push cx
    push dx

    cmp byte [edd_supported], EDD_UNINIT
    jne .cached
    call check_edd

.cached:
    cmp byte [edd_supported], EDD_SUPPORTED
    je .read_edd
    jmp .read_chs

.read_edd:
    ; Read in chunks of up to 64 sectors to avoid BIOS limits
    mov cx, [si + 2]            ; total sectors to read

.edd_loop:
    cmp cx, 0
    je .edd_done

    ; Set chunk size: min(remaining, 64)
    mov ax, cx
    cmp ax, 64
    jbe .edd_chunk_ok
    mov ax, 64
.edd_chunk_ok:
    mov [si + 2], ax            ; update blkcnt in DAP

    push cx                     ; save remaining count
    mov ah, 0x42
    mov dl, [.drive_number]
    int 0x13
    pop cx
    jc .ret                     ; carry set = error, bail out

    ; Advance: subtract sectors read, advance LBA and destination
    movzx eax, word [si + 2]    ; sectors just read
    sub cx, ax                  ; remaining -= chunk

    ; Advance LBA
    add [si + 8], eax           ; d_lba += sectors read

    ; Advance destination address (segment:offset)
    ; Each sector is 512 bytes = 0x200, in segment terms that's 0x20 per sector
    shl ax, 5                   ; ax *= 32 (segments per sector)
    add [si + 6], ax            ; advance segment

    jmp .edd_loop

.edd_done:
    clc                         ; success
    jmp .ret

.read_chs:
    ; Honor the DAP's segment:offset destination (EDD path uses [si+6] too)
    push es
    mov ax, [si + 6]            ; DAP segment
    mov es, ax
    mov bx, [si + 4]            ; DAP offset

    mov ax, [si + 2]            ; blkcnt (number of sectors)
    mov cx, ax                  ; total sectors to read
    mov eax, [si + 8]           ; starting LBA

    ; Convert LBA to CHS and read one sector at a time
    ; (avoids reads crossing track boundaries)
.chs_loop:
    push cx
    push bx
    push eax

    ; LBA to CHS conversion (floppy geometry: 18 sectors/track, 2 heads)
    ; NOTE: ch takes only the low 8 cylinder bits — valid for floppies
    ; (max cyl 79), NOT for large CHS disks.
    xor edx, edx
    mov ecx, 18                 ; sectors per track
    div ecx                     ; eax = LBA / 18, edx = LBA % 18
    mov cl, dl                  ; cl = sector (0-indexed)
    inc cl                      ; cl = sector (1-indexed)

    xor edx, edx
    mov ecx, 2                  ; number of heads
    div ecx                     ; eax = cylinder, edx = head
    mov ch, al                  ; ch = cylinder (low 8 bits)
    mov dh, dl                  ; dh = head

    pop eax                     ; restore LBA for later increment
    pop bx                      ; restore destination offset

    mov ax, 3                   ; retry counter
.chs_retry:
    push ax
    mov ah, 0x02                ; CHS read function
    mov al, 1                   ; read 1 sector
    mov dl, [.drive_number]
    int 0x13                    ; loads into es:bx
    pop ax
    jnc .chs_sector_done        ; success

    ; Reset disk before retry
    push ax
    xor ah, ah                  ; AH=0, reset disk
    mov dl, [.drive_number]
    int 0x13
    pop ax
    dec ax
    jnz .chs_retry

    ; All retries exhausted
    pop cx                      ; clean up sector counter
    pop es                      ; restore caller's es
    stc
    jmp .ret

.chs_sector_done:
    pop cx                      ; restore sector counter

    ; Advance to next sector
    inc eax                     ; next LBA
    add bx, 512                 ; next destination offset

    loop .chs_loop

    pop es                      ; restore caller's es
    clc
    jmp .ret

.ret:
    pop dx
    pop cx
    pop bx
    pop ax
    ret

.drive_number:
    db 0                        ; private to read_disk, static storage for drive number

%endif
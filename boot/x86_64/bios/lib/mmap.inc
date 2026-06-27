;
; Loads the memory map from the BIOS
;

%ifndef __MMAP_INC
%define __MMAP_INC

%include "lib/print_macros.inc"

;
; Detects the system memory map via INT 0x15, EAX=0xE820.
;
; Writes entry count (word) to [MEMORY_MAP_ADDR] and the entries
; (each MEMORY_MAP_ENTRY_SIZE bytes) starting at MEMORY_MAP_ENTRIES_ADDR.
;
; Returns:
;   CF clear on success, CF set on failure (no E820 support).
;   On success, [MEMORY_MAP_ADDR] holds the entry count.
;
; Clobbers: eax, ebx, ecx, edx, di, bp, es is preserved.
;
detect_memory:
    push es
    push si

    ; Entries are addressed via es:di — make sure es is sane.
    xor ax, ax
    mov es, ax

    mov di, MEMORY_MAP_ENTRIES_ADDR
    xor ebx, ebx                    ; continuation = 0 for first call
    xor bp, bp                      ; entry counter

    mov eax, 0xE820
    mov ecx, MEMORY_MAP_ENTRY_SIZE
    mov edx, 0x534D4150             ; 'SMAP'
    int 0x15
    jc .error                       ; first-call carry = no E820
    cmp eax, 0x534D4150             ; BIOS echoes 'SMAP' in eax
    jne .error

    call .validate_entry            ; check ecx, fix up xattr if needed
    jc .skip_first                  ; CF set => entry should be ignored

    inc bp
    add di, MEMORY_MAP_ENTRY_SIZE
.skip_first:
    test ebx, ebx                   ; ebx == 0 -> that was the only entry
    jz .done

.loop:
    ; Fail if the next entry would run past the reserved buffer
    cmp di, MEMORY_MAP_ENTRIES_END - MEMORY_MAP_ENTRY_SIZE + 1
    jae .overflow

    mov eax, 0xE820
    mov ecx, MEMORY_MAP_ENTRY_SIZE
    mov edx, 0x534D4150
    int 0x15
    jc .done                        ; carry on later calls = list ended

    call .validate_entry
    jc .skip

    inc bp
    add di, MEMORY_MAP_ENTRY_SIZE
.skip:
    test ebx, ebx
    jnz .loop

.done:
    mov [MEMORY_MAP_ENTRIES_ADDR], bp
    clc                             ; success
    pop si
    pop es
    ret

.overflow:
    PRINT err_mem_map_overflow
.error:
    pop si
    pop es
    stc                             ; failure
    ret

;
; Validates the entry the BIOS just wrote at es:di.
; Uses ecx (bytes the BIOS actually returned) and, if 24, the xattr dword.
;
; Returns: CF set => caller should ignore this entry (don't count it).
;          CF clear => entry is valid, keep it.
; Clobbers: nothing the caller relies on except flags.
;
.validate_entry:
    ; A zero-length region is meaningless — drop it.
    ; Length is the qword at offset 8 (low dword at di+8, high at di+12).
    mov eax, [es:di + 8]
    or eax, [es:di + 12]
    jz .ignore                         ; length == 0 => ignore

    ; If the BIOS returned a full 24-byte entry, honor the xattr ignore bit.
    cmp ecx, 24
    jne .keep                          ; short entry => no xattr, just keep it
    test byte [es:di + 20], 1          ; bit 0 clear => "ignore this entry"
    jz .ignore

.keep:
    clc
    ret
.ignore:
    stc
    ret

%include "lib/print.inc"

DEFSTRING err_mem_map_overflow, "Memory map overflow", NEWLINE

%endif
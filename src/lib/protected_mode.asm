;
; Switches from 16-bit real mode to 32-bit protected mode and jumps to the kernel.
; Does not return.
;
; Requires:
;   - GDT loaded
;   - Kernel loaded at 0x10000
;

[bits 16]
switch_to_protected_mode:
    cli                             ; disable maskable interrupts

    mov eax, cr0
    or eax, 0x1                     ; set protection enable bit
    mov cr0, eax

    jmp CODE_SEG:protected_mode_entry   ; far jump: flushes pipeline, loads CS

[bits 32]
protected_mode_entry:
    ; Update segment registers to the 32-bit data segment selector.
    ; CS is already set by the far jump above.
    mov ax, DATA_SEG
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    mov esp, 0x7C00                 ; stack just below the bootloader, grows downward

    jmp dword 0x0008:0x10000        ; absolute far jump to kernel entry point

;
; This is the x86_64 BIOS Bootloader entry point.
;
; NOTE: BIOS is no longer the standard, and has largely been replaced with UEFI. This is a learning
; exercise more than anything. As of writing this comment (June 16, 2026), I have not implemented 
; a UEFI bootloader.
;
; The BIOS (Basic Input/Output System) is the system firmware that is responsible for powering on
; the hardware and handing us control. It searches the bootable drives for one whose first sector
; ends with the magic word 0xAA55, and hands off control. This means we must fit our first program
; into 510 bytes.
;
; This lends itself to a multi-stage bootloader, where the first stage just loads another larger
; stage which is responsible for doing more of the setup. This shrinks the responsibility of our
; first stage significantly. All we need to do is read the second stage from the disk and jump.
;

;
; We receive control in 16-bit real mode. The BIOS reads our boot sector in at 0x7C00. The following
; are directives that tell our assembler this information.
;
[bits 16]
[org 0x7C00]

;
; Defines some useful macros in defining strings for printing
;
%include "lib/print_macros.inc"

;
; Our entry point is the first code in the program since we don't do any linking, we just assemble
; into a flat binary.
;
start:
    ;
    ; We start by clearing out segment registers, since the BIOS may leave these in a weird state,
    ; and set up the stack so we can start calling functions.
    ;
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax

    ;
    ; Setting up the stack requires some choices about our memory layout.
    ;
    mov sp, BOOT_STACK_TOP

    ;
    ; The BIOS leaves the drive number in the dl register. We will need this to know which drive
    ; number to read the kernel from. We give the drive number a labeled address because we will
    ; need it later.
    ;
    mov [drive_number], dl

    ;
    ; Print a message that we are starting to boot
    ;
    PRINT msg_boot

    ;
    ; Read stage 2 from disk. The struct containing the info for stage 2 is compiled into the build,
    ; see the `dapack:` label
    ;
    mov si, dapack
    call read_disk
    jc .failed_read

    ;
    ; Jump to stage 2 - storing the drive number on the stack for use in stage 2
    ;
    mov dl, [drive_number]
    push dx
    jmp 0x0000:STAGE2_LOAD_ADDR

.failed_jump:
    PRINT err_failed_stage2_jump
    jmp .hang

.failed_read:
    PRINT err_failed_stage2_read
    jmp .hang

;
; This disables interrupts and infinitely loops the CPU. In a real bootloader, this will not be
; reached
;
.hang:
    cli
    hlt
    jmp $

;
; Disk Argument pack to load stage 2 from disk
;
dapack:
    db 0x10                     ; Size of packet (always 16)
    db 0                        ; Reserved (always 0)
blkcnt: dw STAGE2_SECTORS       ; Number of sectors to load
db_add: dw STAGE2_LOAD_ADDR     ; Offset of address to load to
    dw 0                        ; Segment of address to read to
d_lba: dd 1                     ; Logical Block Address to start reading from
    dd 0                        ; Upper 32-bits of LBA

;
; We store the data the first stage needs here
;
drive_number:
    db 0

;
; Imported functionality from lib files
;
%include "lib/disk.inc"
%include "lib/print.inc"

;
; Strings
;
DEFSTRING msg_boot, NEWLINE, "ChaOS BIOS Bootloader starting...", NEWLINE, NEWLINE
DEFSTRING err_failed_stage2_read, "BOOT ERROR: Failed to read stage 2 from disk", NEWLINE
DEFSTRING err_failed_stage2_jump, "BOOT ERROR: Did not jump to stage 2", NEWLINE

;
; The BIOS identifies the boot sector by a magic number at the end. This adds any necessary padding
; and appends the signature so it can be identified.
;
times 510 - ($ - $$) db 0
dw 0xAA55

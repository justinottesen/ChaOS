;
; Provides macros useful for the print functions
;

%ifndef __PRINT_MACROS_INC
%define __PRINT_MACROS_INC

;
; New line constants
;
CR equ 0x0D
LF equ 0x0A
%define NEWLINE CR, LF

;
; Convenience macro to define strings (& their lengths)
;
%macro DEFSTRING 2+
    %1:         db %2
    %1_len      equ $ - %1
%endmacro

;
; Convenience macro to print strings
;
%macro PRINT 1
    mov si, %1
    mov cx, %1_len
    call print_string
%endmacro

%endif
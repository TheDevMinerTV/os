global start

section .text
bits 32
start:
    mov esp, stack_top

    ; print `OK` to screen
    mov dword [0xb8000], 0x2f4b2f4f

    extern _rust_main
    call _rust_main

    ; print `OKAY` to screen
    mov eax, 0x2f4b2f4f
    mov dword [0xb8000], eax
    mov eax, 0x2f592f41
    mov dword [0xb8004], eax

    hlt

section .bss
stack_bottom:
    ; 2MB stack
    resb 2 * 1024 * 1024
stack_top:

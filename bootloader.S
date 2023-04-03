global start

section .text
bits 32
start:
    mov esp, stack_top
    push ebx
    push eax

    extern _rust_main
    call _rust_main

    pop eax
    pop ebx

    hlt

section .bss
stack_bottom:
    ; 2MB stack
    resb 2 * 1024 * 1024
stack_top:

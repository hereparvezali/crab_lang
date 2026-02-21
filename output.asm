section .data

section .bss

section .text
global _start

_start:
    push rbp
    mov rbp, rsp
    sub rsp, 16

    ; let x = ...
    mov rax, 10
    mov [rbp-8], rax

    ; let y = ...
    mov rax, 15
    mov [rbp-16], rax


    ; default exit
    mov rax, 60
    xor rdi, rdi
    syscall

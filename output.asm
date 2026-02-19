section .data

section .bss

section .text
global _start

_start:
    push rbp
    mov rbp, rsp

    ; exit
    mov rax, 5
    neg rax
    push rax
    mov rax, 5
    neg rax
    pop rbx
    imul rax, rbx
    mov rdi, rax
    mov rax, 60
    syscall


    ; default exit
    mov rax, 60
    xor rdi, rdi
    syscall

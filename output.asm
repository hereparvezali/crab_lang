section .data

section .bss

section .text
global _start

_start:
    push rbp
    mov rbp, rsp

    ; exit
    mov rax, 5
    push rax
    mov rax, 5
    push rax
    mov rax, 5
    pop rbx
    cqo
    idiv rbx
    push rax
    mov rax, 50
    pop rbx
    cqo
    idiv rbx
    pop rbx
    cqo
    idiv rbx
    mov rdi, rax
    mov rax, 60
    syscall


    ; default exit
    mov rax, 60
    xor rdi, rdi
    syscall

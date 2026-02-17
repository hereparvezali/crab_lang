section .text
global _start

_start:
    push rbp
    mov rbp, rsp
    sub rsp, 32
    ; let x = ...
    mov rax, 5
    mov [rbp-8], rax
    ; let y = ...
    mov rax, 2
    push rax
    mov rax, 2
    push rax
    mov rax, 4
    pop rbx
    add rax, rbx
    pop rbx
    cqo
    idiv rbx
    push rax
    mov rax, [rbp-8]
    pop rbx
    add rax, rbx
    mov [rbp-16], rax
    ; let z = ...
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp-8]
    pop rbx
    add rax, rbx
    mov [rbp-24], rax
    ; exit
    mov rax, 2
    push rax
    mov rax, [rbp-24]
    pop rbx
    add rax, rbx
    mov rdi, rax
    mov rax, 60
    syscall

    mov rax, 60
    xor rdi, rdi
    syscall

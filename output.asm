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

    ; if condition
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp-8]
    pop rbx
    cmp rax, rbx
    sete al
    movzx rax, al
    cmp rax, 0
    je .elif_1
    ; then block
    ; exit
    mov rax, [rbp-8]
    mov rdi, rax
    mov rax, 60
    syscall

    jmp .if_end_0
.elif_1:
    ; elif condition
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp-8]
    pop rbx
    cmp rax, rbx
    setg al
    movzx rax, al
    cmp rax, 0
    je .elif_2
    ; elif block
    ; exit
    mov rax, [rbp-16]
    push rax
    mov rax, [rbp-8]
    pop rbx
    sub rax, rbx
    mov rdi, rax
    mov rax, 60
    syscall

    jmp .if_end_0
.elif_2:
    ; else block
    ; exit
    mov rax, [rbp-8]
    push rax
    mov rax, [rbp-16]
    pop rbx
    sub rax, rbx
    mov rdi, rax
    mov rax, 60
    syscall

.if_end_0:


    ; default exit
    mov rax, 60
    xor rdi, rdi
    syscall

.macro isr handler
cli
push rdi
push rax
push rbx
push rbp
push r10
push r13
push r14
push r15

cld

call \handler

mov al, 0x20
out 0x20, al

pop r15
pop r14
pop r13
pop r10
pop rbp
pop rbx
pop rax
pop rdi
sti
iretq
.endm

.global _isr_bus
_isr_bus:
cli
push ax
mov al, 0x20
out 0x20, al
pop ax
sti
iretq

.global _kbrd_isr
_kbrd_isr:
xor rdi, rdi
in al, 0x60
mov dil, al
isr kbrd_handler

.global _pit_isr
_pit_isr:
isr pit_handler



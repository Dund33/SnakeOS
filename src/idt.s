.global _setup_pic
_setup_pic:
push ax

mov al, 0x11
out 0x20, al //start init
out 0xA0, al

mov al, 0x20
out 0x21, al //set offset

mov al, 0x28
out 0xA1, al

mov al, 0x04
out 0x21, al

mov al, 0x02
out 0xA1, al

mov al, 0x01
out 0x21, al //8086 mode
out 0xA1, al

mov	al, 0xFD
out	0x21, al
mov al, 0xFF
out 0xA1, al

pop ax
ret

.global _isr_bus
_isr_bus:
cli
mov al, 0x20
out 0x20, al
sti
iretq

.global _kbrd_isr
_kbrd_isr:
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
xor rdi, rdi
in al, 0x60
mov dil, al
call kbrd_handler

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

.global _load_idt
_load_idt:
cli
lidt [rdi]
sti
ret

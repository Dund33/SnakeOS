.align 8

.global _setup_pic
_setup_pic:
push ax

mov al, 0x11
out 0x20, al //start init

mov al, 0x20
out 0x21, al //set offset

mov al, 0x00
out 0x21, al //no slave pic (for now)

mov al, 0x01
out 0x21, al //8086 mode

mov al, 0xfd
out 0x21, al //mask everything (except kbd)
pop ax
ret

.global _isr_bus
_isr_bus:
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

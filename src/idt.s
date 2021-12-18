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
call farayad
mov al, 0x20
out 0x20, al
iretq

.global _load_idt
_load_idt:
cli
lidt [rdi]
sti
ret

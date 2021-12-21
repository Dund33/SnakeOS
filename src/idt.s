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

mov	al, 0xFC
out	0x21, al
mov al, 0xFF
out 0xA1, al

pop ax
ret

.global _load_idt
_load_idt:
cli
lidt [rdi]
sti
ret

.global _setup_pit
_setup_pit:
cli
push rax
mov al, 0x34            //channel 0, lobyte/hibyte, rate generator
out 0x43, al
mov ax, 11932        //ax = 16 bit reload value
out 0x40,al                       //Set low byte of PIT reload value
mov al,ah                         //ax = high 8 bits of reload value
out 0x40,al
pop rax
sti
ret
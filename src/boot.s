section .multiboot
align 4
header_start:
dd 0xe85250d6                
dd 0                         
dd header_end - header_start
dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
dw 0
dw 0
dd 8
header_end:


extern _kernel
section .text
global _start

_start:
cli
mov esp, stack_end
lgdt [gdtr]
jmp 0x10:stage2


stage2:
mov ax, 0x08      
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax
mov esp, stack_end
cli
call _kernel
hlt
loop:
jmp loop

section .data
gdt:

;null segment
dd 0
dd 0

;data segment
dw 0xffff
dw 0x0
db 0x0
db 0x92
db 0xCF
db 0x0

;code segment
dw 0xffff
dw 0x0
db 0x0
db 0x9A
db 0xCF
db 0x0

gdt_end:

gdtr:
dw gdt_end - gdt - 1
dd gdt

section .bss
align 16
stack:
resb 1048576
stack_end:

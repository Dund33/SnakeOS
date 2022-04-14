.macro isr handler
cli
push eax
push ecx
push edx
call \handler
mov al, 0x20
out 0x20, al
pop edx
pop ecx
pop eax
sti
iretd
.endm

.extern TIME

.global _isr_bus
_isr_bus:
cli
push eax
mov al, 0x20
out 0x20, al
pop eax
sti
iretd

.global _kbrd_isr
_kbrd_isr:
isr kbrd_handler


.global _pit_isr
_pit_isr:
jmp pit_handler



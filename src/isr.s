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

.global _isr_bus
_isr_bus:
cli
push ax
mov al, 0x20
out 0x20, al
pop ax
sti
iretd

.global _kbrd_isr
_kbrd_isr:
isr kbrd_handler


.global _pit_isr
_pit_isr:
cli

push eax
push ebx
push ecx
push edx
push esi
push edi

mov  eax, DWORD PTR [esp+24] //eip
push eax

mov eax, esp
add eax, 40
push eax
lahf
push eax
//pushfd //push flags

jmp pit_handler


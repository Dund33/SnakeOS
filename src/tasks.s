extern test
extern kbrd_server
extern test2
extern swap
global switch_to
global pit_handler

section .text
switch_to:
pop eax; return address
mov eax, [esp]
mov ebx, [eax+40]
push ebx
popfd
mov ebx, [eax+32]
mov [new_ip], ebx
mov ebx, [eax+28]
mov ecx, [eax+24]
mov edx, [eax+20]
mov esi, [eax+16]
mov edi, [eax+12]
mov ebp, [eax+8]
mov esp, [eax+4]
mov eax, [eax]
push eax
mov al, 0x20
out 0x20, al
pop eax
sti
jmp [new_ip]

pit_handler:
cli
mov [tmp], eax
push ebx
push ecx
push edx
push esi
push edi
mov eax, esp
add eax, 28
push ebp
push eax
mov eax, [tmp]
push eax
call swap

section .data
tmp:
resb 4
new_ip:
resb 4
new_esp:
resb 4
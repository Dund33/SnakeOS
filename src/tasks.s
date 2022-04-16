extern CURRENT_PROCESS_PTR
extern swap
global switch
global pit_handler

section .text
switch:
pop eax; return address
mov eax, [CURRENT_PROCESS_PTR]
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
mov eax, [CURRENT_PROCESS_PTR]
mov [eax+4], esp
mov [eax+8], ebp
mov [eax+12], edi
mov [eax+16], esi
mov [eax+20], edx
mov [eax+24], ecx
mov [eax+28], ebx
pop ebx
mov [eax+32], ebx
pop ebx
mov [eax+36], ebx
pop ebx
mov [eax+40], ebx
mov ebx, [tmp]
mov [eax], ebx
call swap

section .data
tmp:
resb 4
new_ip:
resb 4
new_esp:
resb 4
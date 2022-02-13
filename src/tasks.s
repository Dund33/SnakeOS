global pit_handler
extern test
extern test2

pit_handler:
cli

push eax
push ebx
mov eax, [process_num]
imul eax, descriptor_size
add eax, task_descriptors ;descriptor address at eax

mov ebx, [esp + 16]
mov [eax + 32], ebx ;eflags

mov ebx, esp
add ebx, 20
mov [eax + 28], ebx ;esp

mov ebx, [esp + 8]
mov [eax + 24], ebx ;eip

mov [eax], edi ;edi

mov [eax + 4], esi ;esi

mov [eax + 8], edx ;edx

mov [eax + 12], ecx ;ecx

pop ebx
mov [eax + 16], ebx ;ebx

pop ebx
mov [eax + 20], ebx ;eax

mov eax, [process_num]
inc eax
cmp eax, processes
jl num_ok ;its okay lets proceed

mov eax, 0 ;not ok, fix

num_ok: ;now ok
mov [process_num], eax
imul eax, descriptor_size
add eax, task_descriptors ;descriptor address at eax
mov edi, [eax]
mov esi, [eax + 4]
mov edx, [eax + 8]
mov ecx, [eax + 12]
mov ebx, [eax + 16]

push ebx

mov ebx, [eax + 24]
mov [new_ip], ebx

pop ebx

push eax
mov eax, [eax + 32]
cmp eax, 0
je skipped_flags

push eax
popfd

skipped_flags:
pop eax

mov esp, [eax + 28]

mov eax, [eax + 20]

push eax
mov al, 0x20
out 0x20, al
pop eax

sti
jmp [new_ip]

section .data
process_num: db 0
processes: equ 3
descriptor_size equ 36
task_descriptors:
    dd  0
    dd  0
    dd  0
    dd  0
    dd  0
    dd  0 
    dd  0 ;instruction pointer
    dd  0 ;stack pointer
    dd  0 ;flags
    ;-----------
    dd  0
    dd  0
    dd  0
    dd  0
    dd  0
    dd  0
    dd  test ;instruction pointer
    dd  180000h ;stack pointer
    dd  0 ;flags
    ;-----------
    dd  0
    dd  0
    dd  0
    dd  0
    dd  0
    dd  0
    dd  test2 ;instruction pointer
    dd  200000h ;stack pointer
    dd  0 ;flags

new_ip: resb 4


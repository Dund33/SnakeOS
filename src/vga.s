//addr in di (16 bit)
//al - working reg
//bh - address register value
//bl - data register value
.global _move_cursor
_move_cursor:
push ax
push bx
in 0x3DA, al //discard input
in 0x3C0, bh
in 0x3C1, bl
mov al, dh //high bits

mov al, dl //low bits

pop bx
pop ax
ret
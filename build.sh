cargo build --release &&
nasm src/boot.s -o target/x86-custom/release/boot.o -f elf32 &&
nasm src/tasks.s -o target/x86-custom/release/tasks.o -f elf32 &&
cd target/x86-custom/release &&
rm -f SnakeOS.iso &&
rm -f SnakeOS.bin &&
ld -m elf_i386 -T ../../../src/link.ld boot.o tasks.o libSnakeOS.a -o SnakeOS.bin
cd ../../..
cp target/x86-custom/release/SnakeOS.bin iso/boot
grub-mkrescue iso/ -o SnakeOS.iso
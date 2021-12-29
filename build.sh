cargo build --release &&
nasm src/boot.s -o target/x86-custom/release/boot.o -f elf32 &&
cd target/x86-custom/release &&
rm -f SnakeOS.iso &&
rm -f SnakeOS.bin &&
ld -m elf_i386 -T ../../../src/link.ld boot.o libSnakeOS.a -o SnakeOS.bin

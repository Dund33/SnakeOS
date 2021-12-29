BUILD_DIR=build
ROOT_DIR:=$(shell pwd)
RUST_DIR_RELEASE=target/x86-custom/release

kernel:
	cargo build --release && \
    nasm src/boot.s -o $(BUILD_DIR)/boot.o -f elf32 && \
    ld -m elf_i386 -T $(ROOT_DIR)/link.ld $(BUILD_DIR)/boot.o $(RUST_DIR_RELEASE)/libSnakeOS.a -o $(RUST_DIR_RELEASE)/SnakeOS.bin

clean:
	rm -f SnakeOS.iso && \
    rm -f SnakeOS.bin

all:
	kernel clean


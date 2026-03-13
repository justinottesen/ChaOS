.PHONY: all run clean

all: disk.img

disk.img: boot.bin stage2.bin
	cat boot.bin stage2.bin > disk.img

boot.bin: boot.asm
	nasm boot.asm -f bin -o boot.bin

stage2.bin: stage2.asm
	nasm stage2.asm -f bin -o stage2.bin

run: disk.img
	qemu-system-x86_64 -drive format=raw,file=disk.img

clean:
	rm -f boot.bin stage2.bin disk.img

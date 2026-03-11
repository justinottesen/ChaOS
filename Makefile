.PHONY: all run clean
all: boot.bin

boot.bin: boot.asm
	nasm boot.asm -f bin -o boot.bin

run: boot.bin
	qemu-system-x86_64 boot.bin
	
clean:
	rm boot.bin

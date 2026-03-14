.PHONY: all run clean

SRC_DIR=src
BUILD_DIR=build

all: ${BUILD_DIR}/disk.img

${BUILD_DIR}/disk.img: ${BUILD_DIR}/stage1.bin ${BUILD_DIR}/stage2.bin ${BUILD_DIR}/kernel.bin
	cat ${BUILD_DIR}/stage1.bin ${BUILD_DIR}/stage2.bin ${BUILD_DIR}/kernel.bin > ${BUILD_DIR}/disk.img

LIB_ASM=$(wildcard ${SRC_DIR}/lib/*.asm)

${BUILD_DIR}/stage1.bin: ${BUILD_DIR} ${SRC_DIR}/boot/stage1.asm ${LIB_ASM}
	nasm -I ${SRC_DIR} ${SRC_DIR}/boot/stage1.asm -f bin -o ${BUILD_DIR}/stage1.bin

${BUILD_DIR}/stage2.bin: ${BUILD_DIR} ${SRC_DIR}/boot/stage2.asm ${LIB_ASM}
	nasm -I ${SRC_DIR} ${SRC_DIR}/boot/stage2.asm -f bin -o ${BUILD_DIR}/stage2.bin

${BUILD_DIR}/kernel.bin: ${BUILD_DIR} ${SRC_DIR}/kernel/entry.asm
	nasm -I ${SRC_DIR} ${SRC_DIR}/kernel/entry.asm -f bin -o ${BUILD_DIR}/kernel.bin

${BUILD_DIR}:
	mkdir -p ${BUILD_DIR}

run: ${BUILD_DIR}/disk.img
	qemu-system-x86_64 -drive format=raw,file=${BUILD_DIR}/disk.img -no-reboot

run-debug: ${BUILD_DIR}/disk.img
	-qemu-system-x86_64 -drive format=raw,file=${BUILD_DIR}/disk.img -no-reboot -d int,cpu_reset -D ${BUILD_DIR}/qemu.log
	@cat ${BUILD_DIR}/qemu.log

clean:
	rm -rf ${BUILD_DIR}

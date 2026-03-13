.PHONY: all run clean

SRC_DIR=src
BUILD_DIR=build

all: ${BUILD_DIR}/disk.img

${BUILD_DIR}/disk.img: ${BUILD_DIR}/boot.bin ${BUILD_DIR}/stage2.bin
	cat ${BUILD_DIR}/boot.bin ${BUILD_DIR}/stage2.bin > ${BUILD_DIR}/disk.img

${BUILD_DIR}/boot.bin: ${BUILD_DIR} ${SRC_DIR}/boot.asm
	mkdir -p ${BUILD_DIR}
	nasm ${SRC_DIR}/boot.asm -f bin -o ${BUILD_DIR}/boot.bin

${BUILD_DIR}/stage2.bin: ${BUILD_DIR} ${SRC_DIR}/stage2.asm
	mkdir -p ${BUILD_DIR}
	nasm ${SRC_DIR}/stage2.asm -f bin -o ${BUILD_DIR}/stage2.bin

run: ${BUILD_DIR}/disk.img
	qemu-system-x86_64 -drive format=raw,file=${BUILD_DIR}/disk.img

clean:
	rm -rf ${BUILD_DIR}

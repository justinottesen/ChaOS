.PHONY: all run run-debug clean

# --- Paths ---

SRC_DIR   := src
BUILD_DIR := build

BOOT_DIR   := boot
KERNEL_DIR := kernel

BOOT_SRC_DIR   := $(SRC_DIR)/$(BOOT_DIR)
KERNEL_SRC_DIR := $(SRC_DIR)/$(KERNEL_DIR)

BOOT_BUILD_DIR   := $(BUILD_DIR)/$(BOOT_DIR)
KERNEL_BUILD_DIR := $(BUILD_DIR)/$(KERNEL_DIR)

# --- Tools ---

NASM := nasm
QEMU := qemu-system-x86_64

# --- Flags ---

BOOT_NASMFLAGS := -I $(BOOT_SRC_DIR) -f bin
KERNEL_NASMFLAGS := -f bin
QEMUFLAGS := -drive format=raw,file=$(BUILD_DIR)/ChaOS.img -no-reboot

# --- Files ---

IMAGE := $(BUILD_DIR)/ChaOS.img

STAGE1_SRC := $(BOOT_SRC_DIR)/stage1.asm
STAGE2_SRC := $(BOOT_SRC_DIR)/stage2.asm
KERNEL_SRC := $(KERNEL_SRC_DIR)/entry.asm

STAGE1_BIN := $(BOOT_BUILD_DIR)/stage1.bin
STAGE2_BIN := $(BOOT_BUILD_DIR)/stage2.bin
KERNEL_BIN := $(KERNEL_BUILD_DIR)/kernel.bin

BOOT_LIB_ASM := $(wildcard $(BOOT_SRC_DIR)/lib/*.asm)

# --- Main targets ---

all: $(IMAGE)

run: $(IMAGE)
	$(QEMU) $(QEMUFLAGS)

run-debug: $(IMAGE)
	-$(QEMU) $(QEMUFLAGS) -d int,cpu_reset -D $(BUILD_DIR)/qemu.log
	@cat $(BUILD_DIR)/qemu.log

clean:
	rm -rf $(BUILD_DIR)

# --- Image build ---

$(IMAGE): $(STAGE1_BIN) $(STAGE2_BIN) $(KERNEL_BIN) | $(BUILD_DIR)
	cat $+ > $@

# --- Binary builds ---

$(STAGE1_BIN): $(STAGE1_SRC) $(BOOT_LIB_ASM) | $(BOOT_BUILD_DIR)
	$(NASM) $(BOOT_NASMFLAGS) $< -o $@

$(STAGE2_BIN): $(STAGE2_SRC) $(BOOT_LIB_ASM) | $(BOOT_BUILD_DIR)
	$(NASM) $(BOOT_NASMFLAGS) $< -o $@

$(KERNEL_BIN): $(KERNEL_SRC) | $(KERNEL_BUILD_DIR)
	$(NASM) $(KERNEL_NASMFLAGS) $< -o $@

# --- Directory creation ---

$(BUILD_DIR) $(BOOT_BUILD_DIR) $(KERNEL_BUILD_DIR):
	mkdir -p $@

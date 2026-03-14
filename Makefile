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

NASM     := nasm
QEMU     := qemu-system-x86_64
LD       := ld
OBJCOPY  := objcopy

# --- Flags ---

BOOT_NASMFLAGS := -I $(BOOT_SRC_DIR) -f bin
QEMUFLAGS      := -drive format=raw,file=$(BUILD_DIR)/ChaOS.img -no-reboot

# --- Files ---

IMAGE := $(BUILD_DIR)/ChaOS.img

STAGE1_SRC := $(BOOT_SRC_DIR)/stage1.asm
STAGE2_SRC := $(BOOT_SRC_DIR)/stage2.asm

STAGE1_BIN := $(BOOT_BUILD_DIR)/stage1.bin
STAGE2_BIN := $(BOOT_BUILD_DIR)/stage2.bin

BOOT_LIB_ASM := $(wildcard $(BOOT_SRC_DIR)/lib/*.asm)

KERNEL_ENTRY_SRC := $(KERNEL_SRC_DIR)/entry.asm
KERNEL_ENTRY_OBJ := $(KERNEL_BUILD_DIR)/entry.o
KERNEL_LD_SCRIPT := $(KERNEL_SRC_DIR)/kernel.ld
KERNEL_LIB       := $(KERNEL_SRC_DIR)/target/i686-none/debug/libchaos_kernel.a
KERNEL_ELF       := $(KERNEL_BUILD_DIR)/chaos_kernel.elf
KERNEL_BIN       := $(KERNEL_BUILD_DIR)/chaos_kernel.bin

KERNEL_RUST_SRC  := $(wildcard $(KERNEL_SRC_DIR)/src/*.rs)

# --- Main targets ---

all: $(IMAGE)

run: $(IMAGE)
	$(QEMU) $(QEMUFLAGS)

run-debug: $(IMAGE)
	-$(QEMU) $(QEMUFLAGS) -d int,cpu_reset -D $(BUILD_DIR)/qemu.log
	@cat $(BUILD_DIR)/qemu.log

clean:
	rm -rf $(BUILD_DIR)
	cd $(KERNEL_SRC_DIR) && cargo clean

# --- Image build ---

$(IMAGE): $(STAGE1_BIN) $(STAGE2_BIN) $(KERNEL_BIN) | $(BUILD_DIR)
	cat $+ > $@
	truncate -s 1M $@

# --- Binary builds ---

$(STAGE1_BIN): $(STAGE1_SRC) $(BOOT_LIB_ASM) | $(BOOT_BUILD_DIR)
	$(NASM) $(BOOT_NASMFLAGS) $< -o $@

$(STAGE2_BIN): $(STAGE2_SRC) $(BOOT_LIB_ASM) | $(BOOT_BUILD_DIR)
	$(NASM) $(BOOT_NASMFLAGS) $< -o $@

$(KERNEL_BIN): $(KERNEL_ELF) | $(KERNEL_BUILD_DIR)
	$(OBJCOPY) -O binary $< $@

$(KERNEL_ELF): $(KERNEL_ENTRY_OBJ) $(KERNEL_LIB) $(KERNEL_LD_SCRIPT) | $(KERNEL_BUILD_DIR)
	$(LD) -m elf_i386 -T $(KERNEL_LD_SCRIPT) -o $@ \
	    $(KERNEL_ENTRY_OBJ) \
	    --whole-archive $(KERNEL_LIB) --no-whole-archive

$(KERNEL_ENTRY_OBJ): $(KERNEL_ENTRY_SRC) | $(KERNEL_BUILD_DIR)
	$(NASM) -f elf32 $< -o $@

$(KERNEL_LIB): $(KERNEL_RUST_SRC) | $(KERNEL_SRC_DIR)
	cd $(KERNEL_SRC_DIR) && cargo build

# --- Directory creation ---

$(BUILD_DIR) $(BOOT_BUILD_DIR) $(KERNEL_BUILD_DIR):
	mkdir -p $@

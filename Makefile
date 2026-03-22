KERNEL := target/x86_64-unknown-none/debug/kernel
ISO    := iso/chaos.iso

.PHONY: all run clean

all: $(ISO)

$(KERNEL):
	cargo build

$(ISO): $(KERNEL) boot/grub.cfg
	mkdir -p iso/boot/grub
	cp $(KERNEL) iso/boot/kernel
	cp boot/grub.cfg iso/boot/grub/grub.cfg
	nix-shell -p grub2 xorriso --run "grub-mkrescue -o $(ISO) iso 2>/dev/null"
	@echo "Built $(ISO)"

run: $(ISO)
	qemu-system-x86_64 -cdrom $(ISO) -m 64M -no-reboot -no-shutdown

clean:
	rm -rf iso $(ISO)
	cargo clean

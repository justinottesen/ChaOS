//! BIOS boot target: assemble the hand-rolled boot sector with NASM into a
//! flat binary, then boot it directly as a raw disk in QEMU.

use super::Pipeline;
use crate::config::{self, Config};
use crate::memory::MemoryValidator;
use crate::tools;

const BOOT_SRC_DIR: &str = "boot/x86_64/bios";
const STAGE1_SRC: &str = "stage1.asm";
const STAGE2_SRC: &str = "stage2.asm";

/// Workspace package and bare-metal target triple for the kernel build.
const KERNEL_PKG: &str = "kernel";
const KERNEL_TARGET: &str = "x86_64-unknown-none";

const STAGE1: &str = "stage1.bin";
const STAGE2: &str = "stage2.bin";
const IMAGE: &str = "ChaOS.img";

/// Address the BIOS loads the boot sector at
const BOOT_ADDR: u64 = 0x7C00;
// The size of one sector
const SECTOR_SIZE: usize = 512;

pub struct Bios;

impl Pipeline for Bios {
    fn build(&self, config: &Config) {
        // Figure out paths
        let boot_src_dir = config::repo_path(BOOT_SRC_DIR);
        let _stage1_src_path = boot_src_dir.join(STAGE1_SRC);
        let stage2_src_path = boot_src_dir.join(STAGE2_SRC);

        let build_dir = config.build.resolved_build_dir();
        let stage1_bin_path = build_dir.join(STAGE1);
        let stage2_bin_path = build_dir.join(STAGE2);
        let image_path = build_dir.join(IMAGE);

        println!("Building kernel");
        let kernel_path = tools::cargo_build(
            &config::repo_root(),
            KERNEL_PKG,
            KERNEL_TARGET,
            config.build.profile,
            &["core"],
        );
        let kernel_data = std::fs::read(&kernel_path)
            .unwrap_or_else(|e| panic!("Failed to read kernel {}: {e}", kernel_path.display()));

        println!("Creating build dir: {build_dir:?}");
        std::fs::create_dir_all(&build_dir)
            .unwrap_or_else(|e| panic!("could not create build dir {}: {e}", build_dir.display()));

        println!("Building stage 2: {stage2_bin_path:?}");
        tools::nasm(
            &stage2_src_path,
            &stage2_bin_path,
            &[("STAGE2_LOAD_ADDR", BOOT_ADDR + SECTOR_SIZE as u64)],
        );

        let mut stage2_data: Vec<u8> = std::fs::read(&stage2_bin_path)
            .unwrap_or_else(|e| panic!("Failed to read stage 2: {e}"));
        stage2_data.resize(stage2_data.len().next_multiple_of(SECTOR_SIZE), 0);
        let stage2_size = stage2_data.len();
        assert!(
            stage2_size % SECTOR_SIZE == 0,
            "Stage 2 must be a clean number of sectors"
        );
        println!("Stage 2 built (size: {stage2_size})");

        println!("Building stage 1: {stage1_bin_path:?}");
        tools::nasm(
            &config::repo_path(BOOT_SRC_DIR).join(STAGE1_SRC),
            &config.build.resolved_build_dir().join(STAGE1),
            &[
                ("STAGE2_LOAD_ADDR", BOOT_ADDR + SECTOR_SIZE as u64),
                ("BOOT_STACK_TOP", BOOT_ADDR),
                ("STAGE2_SECTORS", (stage2_size / SECTOR_SIZE) as u64),
            ],
        );

        let stage1_data: Vec<u8> = std::fs::read(&stage1_bin_path)
            .unwrap_or_else(|e| panic!("Failed to read stage 2: {e}"));
        assert!(
            stage1_data.len() == SECTOR_SIZE,
            "Boot sector must be exactly {SECTOR_SIZE} bytes"
        );
        assert!(
            stage1_data[510..] == [0x55, 0xAA],
            "Boot sector missing magic signature"
        );

        println!("Creating boot image {image_path:?}");
        let mut image = stage1_data;
        image.extend(stage2_data);
        image.extend(kernel_data);
        image.resize(image.len().next_multiple_of(SECTOR_SIZE), 0);
        assert!(
            image.len() % SECTOR_SIZE == 0,
            "Image must be a clean number of sectors"
        );

        std::fs::write(&image_path, &image)
            .unwrap_or_else(|e| panic!("Failed to create {IMAGE}: {e}"));
        println!("Image size: {} bytes", image.len());

        // During the build, we validate the memory layout to ensure no overlaps
        let memory_validator = MemoryValidator::new(0x100000);
        let claim = |name, start, len| {
            memory_validator
                .claim(name, start, len)
                .unwrap_or_else(|e| panic!("failed to claim memory for {name}: {e}"))
        };

        // Register initial memory layout
        let _bios_reserved_low = claim("Interrupt Table & BIOS Data", 0, 0x4FF);
        let _boot_stage1 = claim("Boot sector", BOOT_ADDR, SECTOR_SIZE as u64);
        let _bios_reserved_high = claim("Video display & BIOS data", 0x80000, 0xFFFFF - 0x80000);

        // Mimic memory layout during execution
        let _stack = claim("Stack", 0x500, BOOT_ADDR - 0x500);
        let _stage2 = claim(
            "Stage 2",
            BOOT_ADDR + SECTOR_SIZE as u64,
            stage2_size.next_multiple_of(SECTOR_SIZE) as u64,
        );
    }

    fn run(&self, config: &Config) {
        let q = &config.qemu;
        tools::qemu_bios(
            &config.build.resolved_build_dir().join(IMAGE),
            &q.memory,
            q.cpus,
            &q.serial,
        );
    }
}

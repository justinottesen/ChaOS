//! BIOS boot target: assemble the hand-rolled boot sector with NASM into a
//! flat binary, then boot it directly as a raw disk in QEMU.

use super::Pipeline;
use crate::config::{self, Config};
use crate::tools;
use std::path::PathBuf;

/// Boot source, relative to the repo root.
const BOOT_SRC: &str = "boot/x86_64/bios/boot.asm";
/// Output artifact name, placed inside the configured build dir.
const BOOT_BIN: &str = "boot.bin";

pub struct Bios;

impl Bios {
    /// Absolute path to the assembled boot binary.
    fn boot_bin(config: &Config) -> PathBuf {
        config.build.resolved_build_dir().join(BOOT_BIN)
    }
}

impl Pipeline for Bios {
    fn build(&self, config: &Config) {
        let build_dir = config.build.resolved_build_dir();
        std::fs::create_dir_all(&build_dir)
            .unwrap_or_else(|e| panic!("could not create build dir {}: {e}", build_dir.display()));

        tools::nasm(&config::repo_path(BOOT_SRC), &Self::boot_bin(config));
    }

    fn run(&self, config: &Config) {
        let q = &config.qemu;
        tools::qemu_bios(&Self::boot_bin(config), &q.memory, q.cpus, &q.serial);
    }
}

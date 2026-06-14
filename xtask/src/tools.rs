//! Thin wrappers over the external tools xtask drives. Each runs a tool
//! as a subprocess and fails loudly if the tool is missing or errors.

use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

/// Assemble a flat binary with NASM (`-f bin`, no linker needed).
pub fn nasm(src: &Path, out: &Path) {
    run(
        "nasm",
        &[
            OsStr::new("-f"),
            OsStr::new("bin"),
            src.as_os_str(),
            OsStr::new("-o"),
            out.as_os_str(),
        ],
    );
}

/// Boot a raw disk image in QEMU.
pub fn qemu_bios(image: &Path, memory: &str, cpus: u32, serial: &str) {
    let drive = format!("format=raw,file={}", image.display());
    let smp = cpus.to_string();
    run(
        "qemu-system-x86_64",
        &[
            OsStr::new("-drive"),
            OsStr::new(&drive),
            OsStr::new("-m"),
            OsStr::new(memory),
            OsStr::new("-smp"),
            OsStr::new(&smp),
            OsStr::new("-serial"),
            OsStr::new(serial),
        ],
    );
}

/// Run a command, inheriting stdio, and panic with context on failure.
fn run(program: &str, args: &[&OsStr]) {
    let status = Command::new(program)
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("failed to launch `{program}`: {e} (is it installed?)"));
    if !status.success() {
        panic!("`{program}` exited with {status}");
    }
}

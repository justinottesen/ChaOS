//! Thin wrappers over the external tools xtask drives. Each runs a tool
//! as a subprocess and fails loudly if the tool is missing or errors.

use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::Command;

/// Assemble a flat binary with NASM (`-f bin`, no linker needed).
pub fn nasm(src: &Path, out: &Path, defines: &[(&str, u64)]) {
    let mut args: Vec<OsString> = vec![
        OsString::from("-f"),
        OsString::from("bin"),
        src.into(),
        OsString::from("-o"),
        out.into(),
    ];
    for (name, value) in defines {
        args.push(format!("-D{name}={value}").into());
    }
    // Resolve `%include` paths relative to the source file's directory.
    // NASM concatenates the -i prefix literally, so the trailing separator matters.
    if let Some(dir) = src.parent() {
        let mut inc = OsString::from("-i");
        inc.push(dir);
        inc.push("/");
        args.push(inc);
    }
    run("nasm", &args);
}

/// Boot a raw disk image in QEMU.
pub fn qemu_bios(image: &Path, memory: &str, cpus: u32, serial: &str) {
    let drive = format!("format=raw,file={}", image.display());
    let smp = cpus.to_string();
    run(
        "qemu-system-x86_64",
        &[
            "-drive",
            drive.as_str(),
            "-m",
            memory,
            "-smp",
            smp.as_str(),
            "-serial",
            serial,
        ],
    );
}

/// Run a command, inheriting stdio, and panic with context on failure.
fn run(program: &str, args: &[impl AsRef<OsStr>]) {
    let status = Command::new(program)
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("failed to launch `{program}`: {e} (is it installed?)"));
    if !status.success() {
        panic!("`{program}` exited with {status}");
    }
}

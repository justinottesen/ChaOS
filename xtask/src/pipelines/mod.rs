//! Per-target build/run pipelines. Each boot target implements `Pipeline`;
//! `select` maps the configured `Target` to its implementation. Adding a
//! boot option is: write a new module, add a `mod` line and a `select` arm.

mod bios;

use crate::config::{Config, Target};

pub trait Pipeline {
    /// Produce the bootable artifact
    fn build(&self, config: &Config);

    /// Boot the artifact in QEMU (assumes `build` already ran)
    fn run(&self, config: &Config);
}

pub fn select(target: Target) -> Box<dyn Pipeline> {
    match target {
        Target::X86Bios => Box::new(bios::Bios),
    }
}

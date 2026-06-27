#![no_std]

use arch_iface::Arch;

pub struct X86_64;

impl Arch for X86_64 {
    fn hang() -> ! {
        loop {
            unsafe { core::arch::asm!("cli; hlt") }
        }
    }
}

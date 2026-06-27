#![no_std]
#![no_main]

use arch_iface::Arch;
use core::panic::PanicInfo;

#[cfg(target_arch = "x86_64")]
use arch_x86_64::X86_64 as Active;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    start_kernel()
}

fn start_kernel() -> ! {
    panic!("Not yet implemented");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        Active::hang();
    }
}

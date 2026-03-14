#![no_std]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let vga = 0xB8000 as *mut u8;
    unsafe {
        *vga = b'O';
        *vga.add(1) = 0x0A; // bright green on black
        *vga.add(2) = b'K';
        *vga.add(3) = 0x0A;
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

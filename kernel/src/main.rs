#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

// --- Multiboot2 Header --------------------------------------------------------

const MULTIBOOT2_MAGIC: u32 = 0xE85250D6;
const MULTIBOOT2_ARCH_I386: u32 = 0;

#[repr(C, align(8))]
struct Multiboot2Header {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
    end_type: u16,
    end_flags: u16,
    end_size: u32,
}

const HEADER_LEN: u32 = core::mem::size_of::<Multiboot2Header>() as u32;

const fn multiboot2_checksum(magic: u32, arch: u32, length: u32) -> u32 {
    0u32.wrapping_sub(magic)
        .wrapping_sub(arch)
        .wrapping_sub(length)
}

#[unsafe(link_section = ".multiboot")]
#[used]
static MULTIBOOT2_HEADER: Multiboot2Header = Multiboot2Header {
    magic: MULTIBOOT2_MAGIC,
    architecture: MULTIBOOT2_ARCH_I386,
    header_length: HEADER_LEN,
    checksum: multiboot2_checksum(MULTIBOOT2_MAGIC, MULTIBOOT2_ARCH_I386, HEADER_LEN),
    end_type: 0,
    end_flags: 0,
    end_size: 8,
};

// --- Kernel Main -------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(_multiboot_info: u64) -> ! {
    // Write directly to the VGA text buffer to confirm we're running in
    // 64-bit Rust. Each cell is two bytes: the character and a colour byte.
    // 0x0f = bright white on black.
    let vga = 0xb8000 as *mut u16;
    let msg = b"Hello from 64-bit Rust!";

    for (i, &byte) in msg.iter().enumerate() {
        unsafe {
            vga.add(i).write_volatile(0x0f00 | byte as u16);
        }
    }

    loop {}
}

// --- Panic Handler -----------------------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

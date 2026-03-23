#![no_std]
#![no_main]

extern crate alloc;

use core::arch::global_asm;
use core::panic::PanicInfo;

mod allocator;
mod kernel_layout;
mod mem;
mod multiboot2;
mod sync;

use allocator::BumpAllocator;
use kernel_layout::PhysMemoryLayout;
use mem::{PhysMemoryMap, PhysRegion};
use multiboot2::{BootInfo, MemoryKind};

global_asm!(include_str!("boot.s"));

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();

// --- Kernel Main -------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(multiboot_info: u64, multiboot_magic: u32) -> ! {
    // Safety: multiboot_info is the bootloader-provided pointer forwarded
    // untouched from EBX via boot.s; multiboot_magic was in EAX.
    let boot_info = unsafe { BootInfo::new(multiboot_info, multiboot_magic) };
    let layout = PhysMemoryLayout::new(&boot_info);

    // Safety: layout.heap is derived from the bootloader memory map with all
    // fixed kernel regions subtracted, guaranteeing no overlap with any other
    // live allocation or data structure.
    unsafe { ALLOCATOR.init(layout.heap.start, layout.heap.size()) };

    let phys_map = PhysMemoryMap::new(
        boot_info
            .memory_map()
            .iter()
            .filter(|r| r.kind == MemoryKind::Available)
            .map(|r| (r.base as usize, r.len as usize)),
    );

    // Safety: VGA text buffer is a well-known MMIO region at 0xB8000.
    // No other code holds a PhysRegion over this range.
    let mut vga: PhysRegion<u16> =
        unsafe { PhysRegion::new(0xb8000 as *mut u16, 80 * 25) };

    let mut writer = VgaWriter::new(&mut vga);
    // Print the full firmware memory map — all region types, for diagnostics.
    writer.print(b"Physical memory map:");
    writer.newline();

    for region in boot_info.memory_map() {
        writer.print(b"  ");
        writer.print_hex(region.base);
        writer.print(b" + ");
        writer.print_hex(region.len);
        writer.print(b"  ");
        writer.print(region.kind.as_str().as_bytes());
        writer.newline();
    }

    // phys_map holds only the Available subset — the regions we can allocate from.
    writer.newline();
    writer.print(b"Available regions: ");
    writer.print_dec(phys_map.unclaimed().count() as u64);

    loop {}
}

// --- VgaWriter ---------------------------------------------------------------
//
// Minimal line-oriented VGA text writer. Temporary until we have a proper
// VGA driver crate.

struct VgaWriter<'a> {
    region: &'a mut PhysRegion<u16>,
    col: usize,
    row: usize,
}

impl<'a> VgaWriter<'a> {
    fn new(region: &'a mut PhysRegion<u16>) -> Self {
        Self { region, col: 0, row: 0 }
    }

    fn put(&mut self, byte: u8) {
        if self.row >= 25 {
            return;
        }
        self.region.write(self.row * 80 + self.col, 0x0f00 | byte as u16);
        self.col += 1;
        if self.col >= 80 {
            self.newline();
        }
    }

    fn newline(&mut self) {
        self.col = 0;
        self.row += 1;
    }

    fn print(&mut self, s: &[u8]) {
        for &b in s {
            self.put(b);
        }
    }

    fn print_hex(&mut self, val: u64) {
        self.print(b"0x");
        for i in (0..16).rev() {
            let nibble = ((val >> (i * 4)) & 0xf) as u8;
            self.put(if nibble < 10 { b'0' + nibble } else { b'a' + nibble - 10 });
        }
    }

    fn print_dec(&mut self, mut val: u64) {
        if val == 0 {
            self.put(b'0');
            return;
        }
        let mut buf = [0u8; 20];
        let mut i = 20;
        while val > 0 {
            i -= 1;
            buf[i] = b'0' + (val % 10) as u8;
            val /= 10;
        }
        self.print(&buf[i..]);
    }
}

// --- Panic Handler -----------------------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Write "!PANIC!" in white-on-red directly to the VGA buffer, bypassing
    // the VGA driver entirely. This works even before the driver is set up.
    let vga = 0xb8000 as *mut u16;
    for (i, &b) in b"!PANIC!".iter().enumerate() {
        unsafe { vga.add(i).write_volatile(0x4f00 | b as u16) };
    }
    loop {}
}

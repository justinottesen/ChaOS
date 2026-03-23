#![allow(dead_code)]

use crate::mem::align_up;
use crate::multiboot2::{BootInfo, MemoryKind};

// --- Linker symbols ----------------------------------------------------------
//
// These are address markers placed by the linker — they have no storage, only
// an address. We use `&raw const` to get that address as a pointer without
// creating a reference, which avoids any alignment or validity requirements.

unsafe extern "C" {
    static __kernel_start: u8;
    static __kernel_end: u8;
    static __boot_stack_bottom: u8;
    static __boot_stack_top: u8;
    static __pml4: u8;
}

// --- Region ------------------------------------------------------------------

/// A contiguous range of physical addresses [start, end).
#[derive(Clone, Copy)]
pub struct Region {
    pub start: usize,
    pub end: usize,
}

impl Region {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "Region: start must be <= end");
        Self { start, end }
    }

    pub fn size(&self) -> usize {
        self.end - self.start
    }

    pub fn overlaps(&self, other: &Region) -> bool {
        self.start < other.end && other.start < self.end
    }
}

// --- PhysMemoryLayout --------------------------------------------------------

/// The authoritative record of how physical memory is partitioned at boot.
///
/// Built once during kernel init from linker symbols and the bootloader memory
/// map. Every other subsystem (allocator, page tables, etc.) derives its
/// working range from this rather than computing addresses independently.
pub struct PhysMemoryLayout {
    /// All kernel sections: text, rodata, data, bss (includes stack and page
    /// tables, which live in .bss).
    pub kernel_image: Region,
    /// Boot stack carved out of .bss for visibility; subregion of kernel_image.
    pub kernel_stack: Region,
    /// Identity-map page tables (PML4 + PDPT + PD) in .bss; subregion of
    /// kernel_image.
    pub page_tables: Region,
    /// Available RAM after all fixed regions; handed to the allocator.
    pub heap: Region,
}

impl PhysMemoryLayout {
    pub fn new(boot_info: &BootInfo) -> Self {
        let kernel_image = Region::new(
            &raw const __kernel_start as usize,
            &raw const __kernel_end as usize,
        );
        let kernel_stack = Region::new(
            &raw const __boot_stack_bottom as usize,
            &raw const __boot_stack_top as usize,
        );
        // PML4, PDPT, and PD are laid out consecutively in .bss, each 4 KiB.
        let page_tables = Region::new(
            &raw const __pml4 as usize,
            &raw const __pml4 as usize + 3 * 4096,
        );
        let heap = find_heap(boot_info, &kernel_image);

        // Sanity: stack and page tables must be subregions of the kernel image.
        assert!(
            kernel_stack.start >= kernel_image.start
                && kernel_stack.end <= kernel_image.end,
            "kernel stack is not within kernel image"
        );
        assert!(
            page_tables.start >= kernel_image.start
                && page_tables.end <= kernel_image.end,
            "page tables are not within kernel image"
        );

        Self { kernel_image, kernel_stack, page_tables, heap }
    }
}

// --- Helpers -----------------------------------------------------------------

/// Find the heap region: the portion of available RAM that follows the kernel
/// image. We locate the available RAM region that contains `kernel_image.end`,
/// then start the heap there (page-aligned).
fn find_heap(boot_info: &BootInfo, kernel_image: &Region) -> Region {
    let heap_start = align_up(kernel_image.end, 4096);

    for region in boot_info.memory_map() {
        if region.kind != MemoryKind::Available {
            continue;
        }
        let r_start = region.base as usize;
        let r_end = r_start + region.len as usize;

        // We want the available region that contains the end of the kernel
        // image — this is the same contiguous RAM block the kernel is in.
        if r_start <= kernel_image.end && kernel_image.end <= r_end {
            return Region::new(heap_start, r_end);
        }
    }

    panic!("no available RAM region found for heap");
}


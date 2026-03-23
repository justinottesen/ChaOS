use crate::mem::PhysRegion;

// --- Header (embedded in the kernel binary) ----------------------------------
//
// The bootloader scans the first 32 KiB of the kernel image for this
// structure. HEADER_MAGIC identifies it; ARCH_I386 is the only defined
// architecture value; the checksum makes the four header fields sum to zero.

#[repr(C, align(8))]
pub struct Multiboot2Header {
    pub magic: u32,
    pub architecture: u32,
    pub header_length: u32,
    pub checksum: u32,
    pub end_type: u16,
    pub end_flags: u16,
    pub end_size: u32,
}

const HEADER_MAGIC: u32 = 0xE85250D6;
const ARCH_I386: u32 = 0;
const HEADER_LEN: u32 = core::mem::size_of::<Multiboot2Header>() as u32;

#[unsafe(link_section = ".multiboot")]
#[used]
static HEADER: Multiboot2Header = Multiboot2Header {
    magic: HEADER_MAGIC,
    architecture: ARCH_I386,
    header_length: HEADER_LEN,
    checksum: header_checksum(HEADER_MAGIC, ARCH_I386, HEADER_LEN),
    end_type: 0,
    end_flags: 0,
    end_size: 8,
};

const fn header_checksum(magic: u32, arch: u32, length: u32) -> u32 {
    0u32.wrapping_sub(magic)
        .wrapping_sub(arch)
        .wrapping_sub(length)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TagType {
    End,
    BootCommandLine,
    BootLoaderName,
    Modules,
    BasicMemoryInfo,
    BiosBootDevice,
    MemoryMap,
    VbeInfo,
    FramebufferInfo,
    ElfSymbols,
    ApmTable,
    Efi32SystemTable,
    Efi64SystemTable,
    SmbiosTables,
    AcpiRsdpV1,
    AcpiRsdpV2,
    NetworkingInfo,
    EfiMemoryMap,
    EfiBootServicesNotTerminated,
    Efi32ImageHandle,
    Efi64ImageHandle,
    ImageLoadBasePhysAddr,
    Unknown(u32),
}

impl TagType {
    fn from_raw(val: u32) -> Self {
        match val {
            0 => Self::End,
            1 => Self::BootCommandLine,
            2 => Self::BootLoaderName,
            3 => Self::Modules,
            4 => Self::BasicMemoryInfo,
            5 => Self::BiosBootDevice,
            6 => Self::MemoryMap,
            7 => Self::VbeInfo,
            8 => Self::FramebufferInfo,
            9 => Self::ElfSymbols,
            10 => Self::ApmTable,
            11 => Self::Efi32SystemTable,
            12 => Self::Efi64SystemTable,
            13 => Self::SmbiosTables,
            14 => Self::AcpiRsdpV1,
            15 => Self::AcpiRsdpV2,
            16 => Self::NetworkingInfo,
            17 => Self::EfiMemoryMap,
            18 => Self::EfiBootServicesNotTerminated,
            19 => Self::Efi32ImageHandle,
            20 => Self::Efi64ImageHandle,
            21 => Self::ImageLoadBasePhysAddr,
            v => Self::Unknown(v),
        }
    }
}

// --- Boot information (provided by the bootloader) ---------------------------
//
// The bootloader passes a pointer to its information structure in EBX and
// places this magic value in EAX to identify itself.

const BOOTLOADER_MAGIC: u32 = 0x36D76289;
const MAX_MEMORY_REGIONS: usize = 64;

/// All information extracted from the multiboot2 info structure, parsed eagerly
/// at construction. Once `BootInfo::new` returns the struct is fully populated;
/// there is no deferred scanning.
pub struct BootInfo {
    memory_map: [MemoryRegion; MAX_MEMORY_REGIONS],
    region_count: usize,
}

impl BootInfo {
    /// Parse the multiboot2 information structure into a `BootInfo`.
    ///
    /// # Safety
    /// `addr` must be the physical address of a valid multiboot2 information
    /// structure as provided by the bootloader in EBX.
    pub unsafe fn new(addr: u64, magic: u32) -> Self {
        assert_eq!(
            magic, BOOTLOADER_MAGIC,
            "not loaded by a multiboot2 bootloader"
        );

        // Safety: addr is the bootloader-provided pointer, trusted to be valid.
        // We need one raw read to learn total_size before we can bound everything.
        let total_size = unsafe { (addr as *const u32).read() } as usize;

        // Safety: addr is valid for total_size bytes per the multiboot2 contract.
        // Every subsequent read goes through read_at, which bounds-checks against
        // total_size — so this single unsafe block covers all parsing below.
        let info = unsafe { PhysRegion::new(addr as *mut u8, total_size) };

        let mut boot_info = Self {
            memory_map: [MemoryRegion {
                base: 0,
                len: 0,
                kind: MemoryKind::Reserved,
            }; MAX_MEMORY_REGIONS],
            region_count: 0,
        };

        let mut offset = 8; // skip the 8-byte info header (total_size + reserved)
        let mut found_end = false;
        while offset < total_size {
            let tag_type = TagType::from_raw(info.read_at(offset));
            let tag_size: usize = info.read_at::<u32>(offset + 4) as usize;

            match tag_type {
                TagType::End => {
                    found_end = true;
                    break;
                }
                TagType::MemoryMap => boot_info.parse_memory_map(&info, offset, tag_size),
                _ => {} // unknown or unhandled tags are intentionally ignored
            }

            // Tags are padded to 8-byte alignment.
            offset += (tag_size + 7) & !7;
        }
        assert!(found_end, "multiboot2: no TagType::End in info structure");

        boot_info
    }

    /// Parse the memory map tag and store all entries.
    ///
    /// Memory map tag layout (offsets relative to tag start):
    ///   +0  type         (u32) — TagType::MemoryMap
    ///   +4  size         (u32) — total tag size in bytes
    ///   +8  entry_size   (u32) — size of each entry, typically 24
    ///   +12 entry_version(u32) — 0
    ///   +16 entries...
    ///
    /// Each entry:
    ///   +0   base_addr (u64)
    ///   +8   length    (u64)
    ///   +16  type      (u32)
    ///   +20  reserved  (u32)
    fn parse_memory_map(&mut self, info: &PhysRegion<u8>, tag_offset: usize, tag_size: usize) {
        let entry_size = info.read_at::<u32>(tag_offset + 8) as usize;
        let mut entry_offset = tag_offset + 16;
        let entries_end = tag_offset + tag_size;

        while entry_offset < entries_end {
            assert!(
                self.region_count < MAX_MEMORY_REGIONS,
                "BootInfo: exceeded MAX_MEMORY_REGIONS; increase the constant"
            );
            let base: u64 = info.read_at(entry_offset);
            let len: u64 = info.read_at(entry_offset + 8);
            let kind: u32 = info.read_at(entry_offset + 16);
            self.memory_map[self.region_count] = MemoryRegion {
                base,
                len,
                kind: MemoryKind::from_raw(kind),
            };
            self.region_count += 1;
            entry_offset += entry_size;
        }
    }

    pub fn memory_map(&self) -> &[MemoryRegion] {
        &self.memory_map[..self.region_count]
    }
}

// --- Memory map types --------------------------------------------------------

#[derive(Clone, Copy)]
pub struct MemoryRegion {
    pub base: u64,
    pub len: u64,
    pub kind: MemoryKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MemoryKind {
    Available,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    Reserved,
}

impl MemoryKind {
    fn from_raw(val: u32) -> Self {
        match val {
            1 => Self::Available,
            3 => Self::AcpiReclaimable,
            4 => Self::AcpiNvs,
            5 => Self::BadMemory,
            _ => Self::Reserved,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::AcpiReclaimable => "ACPI Reclaimable",
            Self::AcpiNvs => "ACPI NVS",
            Self::BadMemory => "Bad Memory",
            Self::Reserved => "Reserved",
        }
    }
}

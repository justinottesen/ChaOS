// These APIs are foundational and will have callers once allocation is wired up.
// The dead_code warning fires because this is a binary crate, where pub doesn't
// suppress it the way it would in a library crate.
#![allow(dead_code)]

/// A validated, exclusively-owned view of a contiguous physical memory region,
/// typed as a sequence of `T`.
///
/// # Uniqueness guarantee
///
/// The only ways to create a `PhysRegion` are:
///   1. `unsafe PhysRegion::new` — you assert the region is valid and unique.
///   2. `PhysRegion::split_at` — safe, consuming, provably non-overlapping.
///
/// Because `PhysRegion` is neither `Copy` nor `Clone`, and `split_at` consumes
/// the parent, the type system guarantees that any two `PhysRegion`s derived
/// from the same root via splits cannot overlap. Compile-time, no tracking.
///
/// For hardware MMIO regions at fixed addresses (e.g. the VGA buffer), the
/// `unsafe` constructor is still necessary until we can derive everything from
/// a root region built from the physical memory map.
///
/// # Volatile access
///
/// All reads and writes use volatile loads and stores. This prevents the
/// compiler from caching values in registers or eliminating writes it considers
/// "dead" — required for MMIO, and a safe default for any memory the kernel
/// shares with hardware.
pub struct PhysRegion<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> PhysRegion<T> {
    /// Construct a `PhysRegion` over `len` elements of type `T` starting at `base`.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - `base` is non-null and aligned for `T`
    /// - The range `[base, base + len * size_of::<T>())` is valid, accessible
    ///   physical memory for the lifetime of this `PhysRegion`
    /// - No other `PhysRegion` (or mutable reference) overlaps this range
    pub unsafe fn new(base: *mut T, len: usize) -> Self {
        Self { ptr: base, len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Split into two non-overlapping regions, consuming `self`.
    ///
    /// Non-overlap is guaranteed by the type system: `self` is consumed and
    /// cannot be used after this call. The two returned regions together cover
    /// exactly the same memory as the original, with no gap and no overlap.
    ///
    /// # Panics
    /// Panics if `mid > self.len()`.
    pub fn split_at(self, mid: usize) -> (Self, Self) {
        // Destructure to consume self without needing a Drop impl.
        let Self { ptr, len } = self;
        assert!(mid <= len, "PhysRegion::split_at: mid out of bounds");
        // Safety: ptr is valid for `len` elements (constructor invariant).
        // mid <= len, so ptr.add(mid) is within the original range.
        // self was consumed by destructuring, so no aliasing from the original.
        unsafe { (Self::new(ptr, mid), Self::new(ptr.add(mid), len - mid)) }
    }
}

impl PhysRegion<u8> {
    /// Read a value of type `U` starting at `byte_offset` within the region.
    ///
    /// Uses an unaligned read since `byte_offset` may not satisfy `U`'s
    /// alignment requirements.
    ///
    /// # Panics
    /// Panics if `byte_offset + size_of::<U>()` exceeds the region length.
    pub fn read_at<U: Copy>(&self, byte_offset: usize) -> U {
        let end = byte_offset + core::mem::size_of::<U>();
        assert!(end <= self.len, "PhysRegion::read_at: out of bounds");
        // Safety: bounds-checked above; ptr is valid per constructor invariant.
        // read_unaligned handles the case where byte_offset doesn't satisfy U's
        // alignment — valid because we own the underlying bytes.
        unsafe { (self.ptr.add(byte_offset) as *const U).read_unaligned() }
    }
}

impl<T: Copy> PhysRegion<T> {
    /// Read element at `index` using a volatile load.
    ///
    /// # Panics
    /// Panics if `index >= self.len()`.
    pub fn read(&self, index: usize) -> T {
        assert!(index < self.len, "PhysRegion::read: index out of bounds");
        // Safety: index is bounds-checked; ptr valid per constructor invariant.
        // Cast to *const T for the read — we hold &self so no concurrent writes.
        unsafe { (self.ptr as *const T).add(index).read_volatile() }
    }

    /// Write `value` to element at `index` using a volatile store.
    ///
    /// # Panics
    /// Panics if `index >= self.len()`.
    pub fn write(&mut self, index: usize, value: T) {
        assert!(index < self.len, "PhysRegion::write: index out of bounds");
        // Safety: index is bounds-checked; &mut self guarantees exclusive access;
        // ptr valid per constructor invariant.
        unsafe { self.ptr.add(index).write_volatile(value) }
    }
}

// --- Utilities ---------------------------------------------------------------

/// Round `addr` up to the nearest multiple of `align`.
/// `align` must be a power of two.
pub(crate) fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

// --- PhysMemoryMap -----------------------------------------------------------

/// Maximum number of physical memory regions the map can hold.
/// Multiboot2 memory maps typically have far fewer entries than this.
const MAX_REGIONS: usize = 32;

/// An inventory of available physical memory regions, built from the
/// bootloader's memory map. Regions can be claimed one at a time; each can
/// only be claimed once, enforced by `Option::take`.
///
/// The `unsafe` is entirely inside `take`: it is justified by the invariants
/// that the regions came from a trusted bootloader and that `Option::take`
/// guarantees each `PhysRegion` is created at most once.
pub struct PhysMemoryMap {
    entries: [Option<(usize, usize)>; MAX_REGIONS],
    count: usize,
}

impl PhysMemoryMap {
    /// Build the map from an iterator of `(base, len)` byte ranges.
    /// Typically called with the available-RAM entries from `BootInfo::memory_map`.
    pub fn new(regions: impl Iterator<Item = (usize, usize)>) -> Self {
        // Option<(usize, usize)> is Copy, so this initialiser is valid.
        let mut map = Self { entries: [None; MAX_REGIONS], count: 0 };
        for region in regions {
            if map.count >= MAX_REGIONS {
                break;
            }
            map.entries[map.count] = Some(region);
            map.count += 1;
        }
        map
    }

    /// Claim the region at `index`, returning an owned `PhysRegion<u8>`.
    ///
    /// Returns `None` if the index is out of bounds or the region has already
    /// been claimed. Each region can be claimed at most once.
    pub fn take(&mut self, index: usize) -> Option<PhysRegion<u8>> {
        let (base, len) = self.entries.get_mut(index)?.take()?;
        // Safety: (base, len) came from the multiboot2 memory map, which the
        // bootloader guarantees describes valid, accessible physical memory.
        // Option::take() above ensures this PhysRegion is created at most once
        // for this entry, so no other PhysRegion can overlap it.
        Some(unsafe { PhysRegion::new(base as *mut u8, len) })
    }

    /// Iterate over regions that have not yet been claimed, as (index, base, len).
    pub fn unclaimed(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        self.entries[..self.count]
            .iter()
            .enumerate()
            .filter_map(|(i, e)| e.map(|(base, len)| (i, base, len)))
    }
}

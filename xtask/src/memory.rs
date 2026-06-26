use std::cell::RefCell;

#[derive(Debug)]
pub enum MemoryError {
    Overlap { with: &'static str, start: u64, end: u64 },
    OutOfRange { end: u64, max: u64 },
    InvalidRange { start: u64, end: u64 },
    Overflow { start: u64, len: u64 },
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MemoryError::Overlap { with, start, end } =>
                write!(f, "region {:#x}..{:#x} overlaps {:?}", start, end, with),
            MemoryError::OutOfRange { end, max } =>
                write!(f, "region ends at {:#x}, beyond max address {:#x}", end, max),
            MemoryError::InvalidRange { start, end } =>
                write!(f, "invalid range {:#x}..{:#x} (start >= end)", start, end),
            MemoryError::Overflow { start, len } =>
                write!(f, "region {:#x} + {:#x} overflows u64", start, len),
        }
    }
}

impl std::error::Error for MemoryError {}

#[derive(Debug, Clone, Copy)]
struct Region {
    id: u64,
    start: u64,
    end: u64, // exclusive
    name: &'static str,
}

struct Inner {
    size: u64,
    next_id: u64,
    regions: Vec<Region>,
}

impl Inner {
    /// Validate [start, start+len) against everything except `ignore`.
    /// Returns the would-be region (without an id assigned yet).
    fn check(&self, start: u64, len: u64, ignore: Option<u64>)
        -> Result<(u64, u64), MemoryError>
    {
        let end = start.checked_add(len)
            .ok_or(MemoryError::Overflow { start, len })?;
        if start >= end {
            return Err(MemoryError::InvalidRange { start, end });
        }
        if end > self.size {
            return Err(MemoryError::OutOfRange { end, max: self.size });
        }
        for r in &self.regions {
            if Some(r.id) == ignore { continue; }
            if start < r.end && r.start < end {
                return Err(MemoryError::Overlap { with: r.name, start, end });
            }
        }
        Ok((start, end))
    }
}

pub struct MemoryValidator {
    inner: RefCell<Inner>,
}

impl MemoryValidator {
    pub fn new(max_address: u64) -> Self {
        Self {
            inner: RefCell::new(Inner {
                size: max_address,
                next_id: 0,
                regions: Vec::new(),
            }),
        }
    }

    pub fn set_max_address(&self, max: u64) {
        self.inner.borrow_mut().size = max;
    }

    pub fn claim(&self, name: &'static str, start: u64, len: u64)
        -> Result<Claim<'_>, MemoryError>
    {
        let mut inner = self.inner.borrow_mut();
        let (start, end) = inner.check(start, len, None)?;
        let id = inner.next_id;
        inner.next_id += 1;
        inner.regions.push(Region { id, start, end, name });
        Ok(Claim { validator: self, id, start, end, name })
    }
}

pub struct Claim<'v> {
    validator: &'v MemoryValidator,
    id: u64,
    start: u64,
    end: u64,
    name: &'static str,
}

impl<'v> Claim<'v> {
    pub fn start(&self) -> u64 { self.start }
    pub fn end(&self) -> u64 { self.end }
    pub fn len(&self) -> u64 { self.end - self.start }
    pub fn name(&self) -> &'static str { self.name }

    /// Move this claim to a new location. Consumes self; on success the
    /// old region is freed and the new one validated. On failure, self is
    /// returned in the Err so the caller keeps the original claim.
    pub fn relocate(self, new_start: u64, len: u64)
        -> Result<Claim<'v>, (Self, MemoryError)>
    {
        let mut inner = self.validator.inner.borrow_mut();

        // Validate the new spot, ignoring our own current region so we
        // don't report an overlap against ourselves.
        let (start, end) = match inner.check(new_start, len, Some(self.id)) {
            Ok(r) => r,
            Err(e) => {
                drop(inner);
                return Err((self, e));
            }
        };

        // Update our region in place — same id, new bounds.
        for r in inner.regions.iter_mut() {
            if r.id == self.id {
                r.start = start;
                r.end = end;
                break;
            }
        }

        let validator = self.validator;
        let id = self.id;
        let name = self.name;
        drop(inner);

        // Suppress self's Drop so it doesn't free the region we just kept.
        std::mem::forget(self);
        Ok(Claim { validator, id, start, end, name })
    }
}

impl<'v> Drop for Claim<'v> {
    fn drop(&mut self) {
        let mut inner = self.validator.inner.borrow_mut();
        let id = self.id;
        inner.regions.retain(|r| r.id != id);
    }
}
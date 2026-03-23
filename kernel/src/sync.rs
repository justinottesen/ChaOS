#![allow(dead_code)]

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

/// A mutual exclusion primitive that busy-waits (spins) until the lock is free.
///
/// # Usage
///
/// ```
/// static FOO: Spinlock<u32> = Spinlock::new(0);
/// let mut guard = FOO.lock();
/// *guard += 1;
/// // lock released here when guard drops
/// ```
///
/// # Deadlocks
///
/// Calling `lock` while already holding the lock on the same CPU will spin
/// forever. Until we have proper interrupt management, interrupt handlers must
/// not acquire any lock held by the interrupted code.
pub struct Spinlock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

/// RAII guard returned by `Spinlock::lock`. Releases the lock on drop.
pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Spin until the lock is acquired, then return a guard.
    pub fn lock(&self) -> SpinlockGuard<'_, T> {
        loop {
            // Try to atomically set locked: false -> true.
            //
            // Acquire ordering: all subsequent reads/writes are guaranteed to
            // happen after this point, so the guard can safely access the data.
            //
            // compare_exchange_weak allows spurious failures, which is fine in
            // a loop and maps to a more efficient instruction on some architectures.
            if self
                .locked
                .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return SpinlockGuard { lock: self };
            }

            // Spin without hammering the bus. On x86 this emits PAUSE, which
            // signals to the CPU that we're in a spin loop, reducing power and
            // pipeline pressure.
            core::hint::spin_loop();
        }
    }
}

impl<T> Drop for SpinlockGuard<'_, T> {
    fn drop(&mut self) {
        // Release ordering: all preceding reads/writes are guaranteed to be
        // visible to the next thread that acquires the lock.
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<T> Deref for SpinlockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: we hold the lock, so no other guard exists for this data.
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinlockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: we hold the lock, so no other guard exists for this data.
        unsafe { &mut *self.lock.data.get() }
    }
}

// Safety: Spinlock provides mutual exclusion, so sharing it across CPUs/
// interrupt handlers is safe as long as T itself can be sent between them.
unsafe impl<T: Send> Sync for Spinlock<T> {}
unsafe impl<T: Send> Send for Spinlock<T> {}

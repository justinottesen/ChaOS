#![no_std]

pub trait Arch {
    fn hang() -> !;
}

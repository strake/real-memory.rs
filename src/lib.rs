#![no_std]

#[cfg(any(target_arch = "x86_64", target_arch = "riscv"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(u64);

impl Address {
    #[cfg(any(target_arch = "x86_64", target_arch = "riscv"))]
    #[inline]
    pub fn new(a: u64) -> Option<Self> { Some(Address(a)) }
}

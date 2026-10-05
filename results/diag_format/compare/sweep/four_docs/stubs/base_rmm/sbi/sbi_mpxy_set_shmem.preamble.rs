use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub const XLEN: usize = 64;

pub struct Flags {
    pub bits: u64,
}

impl Flags {
    pub open spec fn spec_index(self, r: core::ops::Range<usize>) -> u64;
}

pub struct sbiret {
    pub error: i64,
    pub ret: i64,
}

pub struct S {
    pub cmd_input_shmem_phys_lo: UInt64,
    pub cmd_input_shmem_phys_hi: UInt64,
    pub cmd_input_flags: Flags,
    pub shmem_base: u128,
    pub shmem_size: u64,
    pub shmem_enabled: bool,
}

pub open spec fn IsAllOnes(x: UInt64) -> bool;

pub open spec fn IsAligned(x: UInt64, align: int) -> bool;

pub open spec fn ShmemBase(s: S) -> u128;

pub open spec fn ShmemSize(s: S) -> u64;

pub open spec fn ShmemEnabled(s: S) -> bool;

pub open spec fn GetShmemSize() -> u64;

pub open spec fn Concat(hi: UInt64, lo: UInt64) -> u128;

} // verus!

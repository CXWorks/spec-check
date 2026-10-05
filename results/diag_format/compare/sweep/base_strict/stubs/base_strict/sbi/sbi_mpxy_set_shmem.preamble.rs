use vstd::prelude::*;

verus! {

pub type HartId = u64;

pub struct sbiret {
    pub error: i64,
    pub value: (),
}

pub struct S {
    pub shmem_phys_lo: i64,
    pub shmem_phys_hi: i64,
    pub flags: i64,
}

impl S {
    pub open spec fn SharedMemoryBase(self, hart: HartId) -> int;

    pub open spec fn SharedMemorySize(self, hart: HartId) -> int;

    pub open spec fn SharedMemoryDisabled(self, hart: HartId) -> bool;

    pub open spec fn SharedMemorySetupMode(self, hart: HartId) -> i64;
}

pub open spec fn CallingHart() -> HartId;

pub open spec fn ConcatPhysAddr(hi: i64, lo: i64) -> int;

pub open spec fn MpxyGetShmemSize() -> int;

pub open spec fn Bits(x: i64, hi: int, lo: int) -> i64;

} // verus!

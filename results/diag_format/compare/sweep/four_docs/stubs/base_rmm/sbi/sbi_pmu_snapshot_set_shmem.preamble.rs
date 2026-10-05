use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type HartId = u64;

pub struct PmuShmem {
    pub base: u64,
    pub size: u64,
    pub enabled: bool,
    pub cleared: bool,
}

pub struct S {
    pub cmd_input_shmem_phys_lo: u64,
    pub cmd_input_shmem_phys_hi: u64,
}

impl S {
    pub open spec fn PmuSnapshotShmem(self, hart: HartId) -> PmuShmem;
}

pub open spec fn IsAllOnes(x: u64) -> bool;

pub open spec fn CurrentHart() -> HartId;

pub open spec fn PmuSnapshotShmem(hart: HartId) -> PmuShmem;

pub open spec fn AddrIsPageAligned(addr: u64) -> bool;

} // verus!

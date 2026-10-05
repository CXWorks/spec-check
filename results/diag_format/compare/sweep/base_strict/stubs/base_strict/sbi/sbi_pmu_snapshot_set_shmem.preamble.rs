use vstd::prelude::*;

verus! {

pub type HartId = u64;

pub const ALL_ONES: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;

pub struct S {
    pub shmem_phys_lo: u64,
    pub shmem_phys_hi: u64,
}

pub open spec fn CallingHart() -> HartId;

pub open spec fn ConcatXlen(hi: u64, lo: u64) -> int;

pub open spec fn PmuSnapshotShmemEnabled(s: S, hart: HartId) -> bool;

pub open spec fn PmuSnapshotShmemBase(s: S, hart: HartId) -> int;

pub open spec fn PmuSnapshotShmemSize(s: S, hart: HartId) -> int;

pub open spec fn PmuSnapshotShmemLayoutMatchesTable45(s: S, hart: HartId) -> bool;

pub open spec fn PmuSnapshotShmemCleared(s: S, hart: HartId) -> bool;

} // verus!

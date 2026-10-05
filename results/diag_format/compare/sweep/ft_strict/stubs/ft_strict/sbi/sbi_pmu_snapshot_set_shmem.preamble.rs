use vstd::prelude::*;

verus! {

pub type UnsignedLong = u64;

pub type HartId = u64;

pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;

pub const ALL_ONES: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn CallingHart(s: S) -> HartId;

pub open spec fn PmuSnapshotShmemEnabled(s: S, hart: HartId) -> bool;

pub open spec fn PmuSnapshotShmemBase(s: S, hart: HartId) -> int;

pub open spec fn ConcatXlen(s: S, hi: u64, lo: u64) -> int;

pub open spec fn PmuSnapshotShmemSize(s: S, hart: HartId) -> int;

pub open spec fn PmuSnapshotShmemLayoutMatchesTable45(s: S, hart: HartId) -> bool;

pub open spec fn PmuSnapshotShmemCleared(s: S, hart: HartId) -> bool;

} // verus!

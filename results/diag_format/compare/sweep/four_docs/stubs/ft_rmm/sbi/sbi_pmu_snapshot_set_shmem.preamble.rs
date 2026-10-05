use vstd::prelude::*;

verus! {

#[allow(non_camel_case_types)]
pub type long = i64;

#[allow(non_camel_case_types)]
pub type unsigned_long = u64;

pub type SbiCommandReturnCode = i64;

pub type HartId = u64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiCommandReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;
pub const SBI_ERR_DENIED: SbiCommandReturnCode = -4;
pub const SBI_ERR_INVALID_ADDRESS: SbiCommandReturnCode = -5;

pub struct PmuSnapshotShmemState {
    pub base: u64,
    pub size: u64,
    pub enabled: bool,
    pub cleared: bool,
}

pub struct S {
    pub current_hart: HartId,
    pub pmu_snapshot_shmem: Map<HartId, PmuSnapshotShmemState>,
}

pub open spec fn IsAllOnes(s: S, v: u64) -> bool;

pub open spec fn CurrentHart(s: S) -> HartId;

pub open spec fn PmuSnapshotShmem(s: S, hart: HartId) -> PmuSnapshotShmemState;

} // verus!

use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct S {
    pub shmem: u64,
    pub counters: u64,
    pub shmem_snapshot: u64,
    pub overflown_bitmap: u64,
}

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

impl sbiret {
    pub open spec fn is_Ok(self) -> bool;
}

pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_ALREADY_STOPPED: i64 = -8;
pub const SBI_ERR_NO_SHMEM: i64 = -9;

pub const stop_flags: u64 = 1;
pub const counter_idx_base: u64 = 2;
pub const counter_idx_mask: u64 = 3;

pub open spec fn ResultEqual(result: sbiret, code: i64) -> bool;

pub open spec fn SBI_PMU_STOP_FLAG_TAKE_SNAPSHOT(flags: u64) -> bool;

pub open spec fn SBI_PMU_STOP_FLAG_RESET(flags: u64) -> bool;

pub open spec fn stop_flags_reserved(old_s: S, flags: u64) -> bool;

pub open spec fn counter_invalid(old_s: S, base: u64, mask: u64) -> bool;

pub open spec fn counter_already_stopped(old_s: S, base: u64, mask: u64) -> bool;

pub open spec fn shmem_available(old_s: S) -> bool;

pub open spec fn counter_stopped(s: S, base: u64, mask: u64) -> bool;

pub open spec fn counter_reset(new_s: S, base: u64, mask: u64) -> bool;

pub open spec fn snapshot_saved(old_s: S, new_s: S, base: u64, mask: u64) -> bool;

pub open spec fn overflown_bitmap_updated(old_s: S, new_s: S) -> bool;

pub open spec fn snapshot_overflown_bitmap_updated(old_s: S, new_s: S) -> bool;

} // verus!

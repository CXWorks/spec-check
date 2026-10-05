use vstd::prelude::*;

verus! {

pub type UnsignedLong = u64;

pub type SbiRetError = i64;

pub type HartId = u64;

pub struct S {
    pub num_harts: u64,
    pub ipi_pending: Set<HartId>,
    pub sfence_log: Set<(HartId, u64, int)>,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub open spec fn IsValidStartAddr(s: S, start_addr: u64) -> bool;

pub open spec fn IsValidSize(s: S, size: u64) -> bool;

pub open spec fn ResultEqual(result: i64, expected: i64) -> bool;

pub open spec fn TargetHarts(s: S, hart_mask: u64, hart_mask_base: u64) -> Set<HartId>;

pub open spec fn IpiSentToAll(s: S, harts: Set<HartId>) -> bool;

pub open spec fn SfenceVmaExecuted(s: S, hart: HartId, start: u64, end: int) -> bool;

} // verus!

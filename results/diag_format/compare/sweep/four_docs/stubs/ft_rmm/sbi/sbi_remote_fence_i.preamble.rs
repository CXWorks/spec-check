use vstd::prelude::*;

verus! {

pub type UnsignedLong = u64;

pub type unsigned_long = u64;

pub type HartId = u64;

pub type SbiError = i64;

pub type SbiretError = i64;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: i64 = 0;

pub const SBI_ERR_FAILED: i64 = -1;

pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn HartsFromMask(s: S, hart_mask: u64, hart_mask_base: u64) -> Set<HartId>;

pub open spec fn IsHartEnabledByPlatform(s: S, hartid: HartId) -> bool;

pub open spec fn IsHartAvailableToSupervisor(s: S, hartid: HartId) -> bool;

pub open spec fn RequestFailedForUnspecifiedReason(s: S) -> bool;

pub open spec fn ResultEqual(result: i64, code: i64) -> bool;

pub open spec fn IpiSentTo(s: S, hartid: HartId) -> bool;

} // verus!

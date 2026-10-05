use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Bits64 = u64;
pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub struct S {
    pub dummy: u64,
}

pub struct HwTrigger {
    pub tdata1: u64,
    pub tdata2: u64,
    pub tdata3: u64,
}

pub struct DebugTriggerEntry {
    pub trig_state: u64,
}

pub open spec fn TriggerInSet(s: S, trig_idx_base: UInt64, trig_idx_mask: Bits64, i: UInt64) -> bool;

pub open spec fn IsMappedToHwTrigger(s: S, i: UInt64) -> bool;

pub open spec fn ResultEqual(result: SbiErrorCode, code: SbiErrorCode) -> bool;

pub open spec fn trig_max(s: S) -> UInt64;

pub open spec fn PreMappedHwTrigger(s: S, i: UInt64) -> HwTrigger;

pub open spec fn DebugTrigger(s: S, i: UInt64) -> DebugTriggerEntry;

pub open spec fn IsHwTriggerFree(s: S, t: HwTrigger) -> bool;

pub open spec fn IsTrigIdxFree(s: S, i: UInt64) -> bool;

} // verus!

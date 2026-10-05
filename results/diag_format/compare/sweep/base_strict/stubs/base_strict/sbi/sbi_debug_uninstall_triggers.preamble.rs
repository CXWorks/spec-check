use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type Bits64 = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

#[allow(non_upper_case_globals)]
pub const trig_max: UInt64 = 64;

pub struct S {
    pub dummy: u64,
}

pub struct HwTrigger {
    pub tdata1: u64,
    pub tdata2: u64,
    pub tdata3: u64,
}

pub struct DebugTriggerState {
    pub trig_state: u64,
}

pub open spec fn TriggerInSet(trig_idx_base: UInt64, trig_idx_mask: Bits64, i: UInt64) -> bool;

pub open spec fn IsMappedToHwTrigger(i: UInt64) -> bool;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn PreMappedHwTrigger(i: UInt64) -> HwTrigger;

pub open spec fn DebugTrigger(i: UInt64) -> DebugTriggerState;

pub open spec fn IsHwTriggerFree(t: HwTrigger) -> bool;

pub open spec fn IsTrigIdxFree(i: UInt64) -> bool;

} // verus!

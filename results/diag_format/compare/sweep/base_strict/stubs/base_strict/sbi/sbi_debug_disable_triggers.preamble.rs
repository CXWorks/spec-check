use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type long = i64;

pub struct S {
    pub dummy: u64,
}

pub struct HwTrigger {
    pub vs: u64,
    pub vu: u64,
    pub s: u64,
    pub u: u64,
}

pub const SBI_SUCCESS: long = 0;

pub const SBI_ERR_INVALID_PARAM: long = -3;

pub const trig_idx_base: UInt64 = 0;

pub const trig_idx_mask: UInt64 = 1;

pub open spec fn TriggerInSet(s: S, base: UInt64, mask: UInt64, idx: UInt64) -> bool;

pub open spec fn IsMappedToHwTrigger(idx: UInt64) -> bool;

pub open spec fn TrigMax() -> UInt64;

pub open spec fn MappedHwTrigger(idx: UInt64) -> HwTrigger;

pub open spec fn ResultEqual(result: long, code: long) -> bool;

} // verus!

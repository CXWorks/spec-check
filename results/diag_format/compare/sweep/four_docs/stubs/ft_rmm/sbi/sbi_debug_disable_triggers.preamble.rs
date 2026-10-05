use vstd::prelude::*;

verus! {

#[allow(non_camel_case_types)]
pub type unsigned_long = u64;

#[allow(non_camel_case_types)]
pub type sbiret = i64;

pub const SBI_SUCCESS: sbiret = 0;
pub const SBI_ERR_INVALID_PARAM: sbiret = -3;

pub const trig_max: unsigned_long = 64;

pub struct HwTrigger {
    pub vs: u64,
    pub vu: u64,
    pub s: u64,
    pub u: u64,
}

pub struct S {
    pub triggers: Map<unsigned_long, HwTrigger>,
    pub mapped: Set<unsigned_long>,
}

pub open spec fn TriggerSet(s: S, trig_idx_base: unsigned_long, trig_idx_mask: unsigned_long) -> Set<unsigned_long>;

pub open spec fn IsMappedToHwTrigger(s: S, trig_idx: unsigned_long) -> bool;

pub open spec fn HwTriggerOf(s: S, trig_idx: unsigned_long) -> HwTrigger;

pub open spec fn ResultEqual(result: sbiret, code: sbiret) -> bool;

} // verus!

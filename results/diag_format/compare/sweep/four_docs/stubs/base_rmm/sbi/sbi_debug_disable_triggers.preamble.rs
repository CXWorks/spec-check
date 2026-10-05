use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type SbiError = i64;

pub struct sbiret {
    pub error: SbiError,
    pub value: UInt,
}

pub struct S {
    pub dummy: u64,
}

pub struct HwTrigger {
    pub vs: u64,
    pub vu: u64,
    pub s: u64,
    pub u: u64,
}

pub spec const SBI_SUCCESS: SbiError = 0;
pub spec const SBI_ERR_INVALID_PARAM: SbiError = -3;

pub spec const trig_max: UInt = 64;

pub open spec fn TriggerSet(trig_idx_base: UInt, trig_idx_mask: UInt) -> Set<UInt>;

pub open spec fn IsMappedToHwTrigger(trig_idx: UInt) -> bool;

pub open spec fn ResultEqual(result: sbiret, code: SbiError) -> bool;

pub open spec fn HwTriggerOf(trig_idx: UInt) -> HwTrigger;

} // verus!

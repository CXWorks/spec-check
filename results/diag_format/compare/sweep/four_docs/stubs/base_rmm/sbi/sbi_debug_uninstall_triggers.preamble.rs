use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type TrigState = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub struct HwTrigger {
    pub tdata1: UInt,
    pub tdata2: UInt,
    pub tdata3: UInt,
}

pub struct Trigger {
    pub trig_state: TrigState,
}

pub spec const SBI_SUCCESS: i64 = 0;
pub spec const SBI_ERR_INVALID_PARAM: i64 = -3;

pub spec const CLEARED: TrigState = 0;

pub spec const trig_max: UInt = 64;

pub open spec fn TriggerSet(trig_idx_base: UInt, trig_idx_mask: UInt) -> Set<UInt>;

pub open spec fn IsMappedToHwTrigger(trig_idx: UInt) -> bool;

pub open spec fn ResultEqual(result: sbiret, code: i64) -> bool;

pub open spec fn HwTriggerOf(trig_idx: UInt) -> HwTrigger;

pub open spec fn TriggerAt(trig_idx: UInt) -> Trigger;

pub open spec fn IsFreeTrigIdx(trig_idx: UInt) -> bool;

pub open spec fn IsFreeHwTrigger(hw: HwTrigger) -> bool;

} // verus!

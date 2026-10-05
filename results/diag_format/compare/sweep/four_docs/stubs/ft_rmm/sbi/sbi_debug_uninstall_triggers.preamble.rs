use vstd::prelude::*;

verus! {

pub type unsigned_long = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct HwTrigger {
    pub tdata1: u64,
    pub tdata2: u64,
    pub tdata3: u64,
}

pub type TrigState = u64;

pub struct Trigger {
    pub trig_state: TrigState,
}

pub struct S {
    pub triggers: Seq<Trigger>,
    pub hw_triggers: Seq<HwTrigger>,
}

pub spec const SBI_SUCCESS: sbiret = sbiret { error: 0, value: 0 };

pub spec const SBI_ERR_INVALID_PARAM: sbiret = sbiret { error: -3, value: 0 };

pub spec const CLEARED: TrigState = 0;

pub spec const trig_max: u64 = 32;

pub open spec fn TriggerSet(s: S, trig_idx_base: u64, trig_idx_mask: u64) -> Set<u64>;

pub open spec fn IsMappedToHwTrigger(s: S, trig_idx: u64) -> bool;

pub open spec fn ResultEqual(a: sbiret, b: sbiret) -> bool;

pub open spec fn HwTriggerOf(s: S, trig_idx: u64) -> HwTrigger;

pub open spec fn TriggerAt(s: S, trig_idx: u64) -> Trigger;

pub open spec fn IsFreeTrigIdx(s: S, trig_idx: u64) -> bool;

pub open spec fn IsFreeHwTrigger(s: S, hw: HwTrigger) -> bool;

} // verus!

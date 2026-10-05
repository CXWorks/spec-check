use vstd::prelude::*;

verus! {

pub type UnsignedLong = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct HwTrigger {
    pub vs: bool,
    pub vu: bool,
    pub s: bool,
    pub u: bool,
}

pub struct TrigState {
    pub saved: HwTrigger,
}

pub struct S {
    pub triggers: Map<u64, TrigState>,
}

pub spec const SBI_SUCCESS: sbiret = sbiret { error: 0, value: 0 };

pub spec const SBI_ERR_INVALID_PARAM: sbiret = sbiret { error: -3, value: 0 };

pub spec const trig_max: u64 = 64;

pub open spec fn TrigSet(trig_idx_base: u64, trig_idx_mask: u64) -> Set<u64>;

pub open spec fn IsMappedToHwTrigger(trig_idx: u64) -> bool;

pub open spec fn MappedHwTrigger(trig_idx: u64) -> HwTrigger;

pub open spec fn trig_state(trig_idx: u64) -> TrigState;

pub open spec fn ResultEqual(result: sbiret, expected: sbiret) -> bool;

} // verus!

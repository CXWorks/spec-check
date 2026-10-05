use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type Int = i64;

pub struct sbiret {
    pub error: Int,
    pub value: UInt,
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
    pub triggers: Seq<TrigState>,
    pub hw_triggers: Seq<HwTrigger>,
}

pub spec const SBI_SUCCESS: Int = 0;

pub spec const SBI_ERR_INVALID_PARAM: Int = -3;

pub spec const trig_max: UInt = 64;

pub open spec fn TrigSet(trig_idx_base: UInt, trig_idx_mask: UInt) -> Set<UInt>;

pub open spec fn IsMappedToHwTrigger(trig_idx: UInt) -> bool;

pub open spec fn ResultEqual(result: sbiret, code: Int) -> bool;

pub open spec fn MappedHwTrigger(s: S, trig_idx: UInt) -> HwTrigger;

pub open spec fn trig_state(s: S, trig_idx: UInt) -> TrigState;

} // verus!

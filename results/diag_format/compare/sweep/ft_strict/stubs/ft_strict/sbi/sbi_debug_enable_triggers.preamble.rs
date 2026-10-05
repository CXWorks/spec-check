use vstd::prelude::*;
verus! {

pub type UInt = u64;
pub type Bitmask = u64;
pub type SbiErrorCode = i64;
pub type HartId = u64;

pub struct S {
    pub dummy: u64,
}

pub struct TriggerConfig {
    pub vs: bool,
    pub vu: bool,
    pub s: bool,
    pub u: bool,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;
pub const result: SbiErrorCode = -100;

pub open spec fn InTriggerSet(s: S, trig_idx_base: UInt, trig_idx_mask: Bitmask, i: UInt) -> bool;
pub open spec fn IsMappedToHwTrigger(s: S, hart: HartId, i: UInt) -> bool;
pub open spec fn CallingHart() -> HartId;
pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;
pub open spec fn trig_max(s: S) -> UInt;
pub open spec fn MappedHwTrigger(s: S, hart: HartId, i: UInt) -> TriggerConfig;
pub open spec fn TrigState(s: S, hart: HartId, i: UInt) -> TriggerConfig;

} // verus!

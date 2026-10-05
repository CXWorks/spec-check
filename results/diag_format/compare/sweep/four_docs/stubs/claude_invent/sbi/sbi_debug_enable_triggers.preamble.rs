use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

pub struct S {
    pub dummy: u64,
}

pub struct TriggerConfig {
    pub vs: bool,
    pub vu: bool,
    pub s: bool,
    pub u: bool,
}

pub open spec fn TrigMax(s: S) -> u64;

pub open spec fn TriggerIsMapped(s: S, idx: int) -> bool;

pub open spec fn MappedHwTrigger(s: S, idx: int) -> TriggerConfig;

pub open spec fn TrigState(s: S, idx: int) -> TriggerConfig;

} // verus!

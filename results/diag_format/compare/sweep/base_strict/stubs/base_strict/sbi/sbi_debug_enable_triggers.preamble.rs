use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub type HartId = u64;

pub struct S {
    pub dummy: int,
}

pub struct TriggerConfig {
    pub vs: bool,
    pub vu: bool,
    pub s: bool,
    pub u: bool,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub const SBI_ERR_INVALID_PARAM: SbiErrorCode = -3;

#[allow(non_upper_case_globals)]
pub const trig_idx_base: UInt = 0;

#[allow(non_upper_case_globals)]
pub const trig_idx_mask: UInt = 1;

#[allow(non_upper_case_globals)]
pub const trig_max: UInt = 2;

pub open spec fn InTriggerSet(base: UInt, mask: UInt, i: UInt) -> bool;

pub open spec fn IsMappedToHwTrigger(hart: HartId, i: UInt) -> bool;

pub open spec fn CallingHart() -> HartId;

pub open spec fn ResultEqual(error: SbiErrorCode, code: SbiErrorCode) -> bool;

pub open spec fn MappedHwTrigger(hart: HartId, i: UInt) -> TriggerConfig;

pub open spec fn TrigState(hart: HartId, i: UInt) -> TriggerConfig;

} // verus!

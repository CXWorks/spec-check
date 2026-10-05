use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn DebugTriggerIsMapped(s: S, idx: int) -> bool;

pub open spec fn DebugTriggerMax(s: S) -> u64;

pub open spec fn HwTriggerVsBit(s: S, idx: int) -> u64;

pub open spec fn HwTriggerVuBit(s: S, idx: int) -> u64;

pub open spec fn HwTriggerSBit(s: S, idx: int) -> u64;

pub open spec fn HwTriggerUBit(s: S, idx: int) -> u64;

pub open spec fn DebugTriggerStateEqual(old_s: S, new_s: S, idx: int) -> bool;

} // verus!

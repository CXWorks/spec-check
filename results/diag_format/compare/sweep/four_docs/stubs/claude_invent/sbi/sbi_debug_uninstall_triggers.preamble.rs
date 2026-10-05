use vstd::prelude::*;

verus! {

pub struct SbiRet {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub open spec fn DebugTrigMax(s: S) -> int;
pub open spec fn DebugTrigIsMapped(s: S, idx: int) -> bool;
pub open spec fn DebugTrigMappedHwTrigger(s: S, idx: int) -> int;
pub open spec fn DebugTrigState(s: S, idx: int) -> int;
pub open spec fn DebugTrigIdxIsFree(s: S, idx: int) -> bool;
pub open spec fn HwTriggerTdata1(s: S, hw: int) -> int;
pub open spec fn HwTriggerTdata2(s: S, hw: int) -> int;
pub open spec fn HwTriggerTdata3(s: S, hw: int) -> int;
pub open spec fn HwTriggerIsFree(s: S, hw: int) -> bool;

} // verus!

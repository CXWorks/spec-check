use vstd::prelude::*;

verus! {

pub type SbiReturnCode = i64;

pub const SBI_SUCCESS: SbiReturnCode = 0;

pub struct S {
    pub stime: u64,
}

pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

pub open spec fn NextTimerEventTime() -> u64;

pub open spec fn IsTimeInFuture(t: u64) -> bool;

pub open spec fn stime_value(s: S) -> u64;

} // verus!

use vstd::prelude::*;
verus! {

pub type SbiReturnCode = i64;

pub const SBI_SUCCESS: SbiReturnCode = 0;

pub spec const stime_value: u64 = 1;

pub struct S {
    pub stimecmp: u64,
    pub timer_pending: bool,
}

pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

pub open spec fn NextTimerEventTime() -> u64;

pub open spec fn CurrentTime() -> u64;

pub open spec fn TimerInterruptPending() -> bool;

} // verus!

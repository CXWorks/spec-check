use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type SbiReturnCode = i64;

pub const SBI_SUCCESS: SbiReturnCode = 0;
pub const SBI_ERR_FAILED: SbiReturnCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiReturnCode = -3;

pub struct NextTimerEventTime {}

pub struct TimerInterruptPending {}

pub struct S {
    pub next_timer_event_time: UInt64,
    pub timer_interrupt_pending: bool,
    pub current_time: UInt64,
}

pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

pub open spec fn NextTimerEventTime(s: S) -> UInt64;

pub open spec fn TimerInterruptPending(s: S) -> bool;

pub open spec fn CurrentTime(s: S) -> UInt64;

} // verus!

use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type SbiReturnCode = i64;

pub const SBI_SUCCESS: SbiReturnCode = 0;

pub struct S {
    pub next_timer_event_time: UInt64,
    pub timer_interrupt_pending: bool,
    pub current_time: UInt64,
}

pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

pub open spec fn NextTimerEventTime(s: S) -> UInt64;

pub open spec fn IsTimeInFuture(s: S, t: UInt64) -> bool;

pub open spec fn TimerInterruptPending(s: S) -> bool;

} // verus!

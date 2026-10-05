use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u32;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsRegisteredForClockRateNotification(s: S, agent: u32, clock_id: u32) -> bool;

pub open spec fn NotificationSentTo(s: S, agent: u32) -> bool;

pub open spec fn ClockRateTransitionComplete(s: S, clock_id: u32) -> bool;

pub open spec fn ClockRate(s: S, clock_id: u32) -> u64;

pub open spec fn AgentThatCausedRateChange(s: S, clock_id: u32) -> u32;

} // verus!

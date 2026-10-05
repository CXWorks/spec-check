use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub recipient_agent: UInt32,
}

pub open spec fn RecipientAgent(s: S) -> UInt32;

pub open spec fn IsRegisteredForClockRateChangeNotification(s: S, agent: UInt32, clock_id: UInt32) -> bool;

pub open spec fn ClockRateChangedByOtherAgentOrPlatform(s: S, agent_id: UInt32, recipient: UInt32) -> bool;

pub open spec fn ClockRateTransitionCompleted(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockRate(s: S, clock_id: UInt32) -> int;

pub open spec fn RateLow(rate: [UInt32; 2]) -> int;

pub open spec fn RateHigh(rate: [UInt32; 2]) -> int;

} // verus!

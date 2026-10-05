use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const recipient_agent: UInt32 = 0;

pub open spec fn IsRegisteredForClockRateChangeNotification(s: S, agent: UInt32, clock_id: UInt32) -> bool;

pub open spec fn NotificationSentTo(s: S, agent: UInt32) -> bool;

pub open spec fn ClockRateTransitionComplete(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockRate(s: S, clock_id: UInt32) -> UInt32;

pub open spec fn AgentThatCausedRateChange(s: S, clock_id: UInt32) -> UInt32;

} // verus!

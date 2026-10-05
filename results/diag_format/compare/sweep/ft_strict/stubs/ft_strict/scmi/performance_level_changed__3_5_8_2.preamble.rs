use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const recipient_agent: UInt32 = 0;

pub open spec fn IsSubscribedToPerfLevelNotifications(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn PerformanceLevelChanged(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerformanceLevel(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PerfLevelChangeInitiator(s: S, domain_id: UInt32) -> UInt32;

} // verus!

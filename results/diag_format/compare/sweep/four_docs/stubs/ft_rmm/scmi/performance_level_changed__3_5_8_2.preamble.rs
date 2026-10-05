use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const recipient: UInt32 = 0;

pub open spec fn AgentIsSubscribedToLevelChange(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn PerformanceLevelChanged(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerformanceLevelOf(s: S, domain_id: UInt32) -> UInt32;

} // verus!

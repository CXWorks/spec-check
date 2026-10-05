use vstd::prelude::*;

verus! {

pub type UInt = u64;
pub type UInt32 = u32;
pub type RmiStatusCode = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub spec const recipient: UInt32 = 0;

pub open spec fn AgentIsSubscribedToLevelChange(s: S, agent: UInt32, domain_id: UInt32) -> bool;

pub open spec fn PerformanceLevelChanged(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerformanceLevelOf(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

} // verus!

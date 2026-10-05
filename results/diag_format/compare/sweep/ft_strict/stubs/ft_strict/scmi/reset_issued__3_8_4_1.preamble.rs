use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

#[allow(non_upper_case_globals)]
pub const recipient_agent: UInt32 = 1;

pub open spec fn ResetDomainHasBeenReset(s: S, domain_id: UInt32, reset_state: UInt32) -> bool;

pub open spec fn AgentRegisteredForResetNotifications(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn ResetCausedByAgent(s: S, domain_id: UInt32, agent_id: UInt32) -> bool;

} // verus!

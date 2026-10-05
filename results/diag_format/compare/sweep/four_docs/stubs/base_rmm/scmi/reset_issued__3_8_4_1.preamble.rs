use vstd::prelude::*;

verus! {

pub type RmiStatusCode = u64;
pub type AgentId = u64;
pub type DomainId = u64;
pub type ResetState = u64;

pub spec const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub spec const agent_id: AgentId = 10;
pub spec const domain_id: DomainId = 20;
pub spec const reset_state: ResetState = 30;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsRegisteredForResetNotification(s: S, agent: AgentId, domain: DomainId) -> bool;

pub open spec fn ResetDomainHasBeenReset(s: S, domain: DomainId, state: ResetState) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

} // verus!

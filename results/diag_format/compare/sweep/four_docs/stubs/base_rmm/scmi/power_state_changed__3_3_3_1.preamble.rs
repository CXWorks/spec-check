use vstd::prelude::*;

verus! {

pub enum RsiCommandReturnCode {
    RsiSuccess,
    RsiErrorInput,
    RsiErrorState,
    RsiIncomplete,
}

pub struct PowerDomain {
    pub state: u32,
}

pub struct S {
    pub power_domains: Map<u32, PowerDomain>,
    pub notifications: Set<u32>,
}

pub open spec fn IsRegisteredForPowerStateNotification(s: S, recipient_agent: u32, domain_id: u32) -> bool;

pub open spec fn NotificationSentTo(s: S, recipient_agent: u32) -> bool;

pub open spec fn PowerStateTransitionCompleted(s: S, domain_id: u32) -> bool;

pub open spec fn PowerDomainAt(s: S, domain_id: u32) -> PowerDomain;

pub open spec fn PowerTransitionCausedBy(s: S, domain_id: u32, agent_id: u32) -> bool;

} // verus!

use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct PowerDomain {
    pub state: UInt32,
}

pub struct S {
    pub dummy: int,
}

pub spec const recipient_agent: UInt32 = 1;

pub open spec fn IsRegisteredForPowerStateNotification(s: S, agent: UInt32, domain_id: UInt32) -> bool;

pub open spec fn NotificationSentTo(s: S, agent: UInt32) -> bool;

pub open spec fn PowerStateTransitionCompleted(s: S, domain_id: UInt32) -> bool;

pub open spec fn PowerDomainAt(s: S, domain_id: UInt32) -> PowerDomain;

pub open spec fn PowerTransitionCausedBy(s: S, domain_id: UInt32, agent_id: UInt32) -> bool;

} // verus!

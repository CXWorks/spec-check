use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct Recipient {
    pub id: u32,
}

pub struct S {
    pub dummy: u32,
}

pub open spec fn AgentRegisteredForPowerStateNotification(s: S, recipient: Recipient, domain_id: UInt32) -> bool;

pub open spec fn NotificationSentTo(s: S, recipient: Recipient, domain_id: UInt32) -> bool;

pub open spec fn PowerStateTransitionCompleted(s: S, domain_id: UInt32, power_state: UInt32) -> bool;

pub open spec fn PowerTransitionCausedBy(s: S, domain_id: UInt32, agent_id: UInt32) -> bool;

} // verus!

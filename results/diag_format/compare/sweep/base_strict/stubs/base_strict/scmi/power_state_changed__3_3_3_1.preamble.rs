use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type AgentId = u32;

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
    RmiErrorDevice,
    RmiErrorNotSupported,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn AgentRegisteredForPowerStateNotification(s: S, recipient: AgentId, domain_id: UInt32) -> bool;

pub open spec fn NotificationSentTo(s: S, recipient: AgentId, domain_id: UInt32) -> bool;

pub open spec fn PowerStateTransitionCompleted(old_s: S, new_s: S, domain_id: UInt32, power_state: UInt32) -> bool;

pub open spec fn PowerTransitionCausedBy(old_s: S, new_s: S, domain_id: UInt32, agent_id: UInt32) -> bool;

} // verus!

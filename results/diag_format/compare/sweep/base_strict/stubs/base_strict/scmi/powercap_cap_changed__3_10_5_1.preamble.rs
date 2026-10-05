use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type AgentId = u64;

pub struct S {
    pub dummy: int,
}

pub open spec fn AgentRegisteredForCapChangeNotification(s: S, recipient: AgentId, domain_id: UInt32) -> bool;

pub open spec fn NotificationSent(s: S, recipient: AgentId, domain_id: UInt32) -> bool;

pub open spec fn PowerCapTransitionCompleted(s: S, domain_id: UInt32) -> bool;

pub open spec fn AgentCausingChange(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn DomainPowerCap(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn DomainCai(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn DomainSupportsCpc(s: S, domain_id: UInt32) -> bool;

} // verus!

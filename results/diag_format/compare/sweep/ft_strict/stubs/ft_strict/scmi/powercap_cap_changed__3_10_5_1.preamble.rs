use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

#[allow(non_upper_case_globals)]
pub const recipient: UInt32 = 1;

pub open spec fn AgentRegisteredForCapChangeNotification(s: S, recipient_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn NotificationSent(s: S, recipient_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn PowerCapTransitionCompleted(s: S, domain_id: UInt32) -> bool;

pub open spec fn AgentCausingChange(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn DomainPowerCap(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn DomainCai(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn DomainSupportsCpc(s: S, domain_id: UInt32) -> bool;

} // verus!

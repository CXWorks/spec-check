use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const recipient: UInt32 = 1;

pub spec const POWERCAP_CAP_CHANGED: UInt32 = 2;

pub open spec fn IsRegisteredForPowerCapNotification(s: S, agent: UInt32, domain_id: UInt32) -> bool;

pub open spec fn NotificationSent(s: S, agent: UInt32, notification: UInt32) -> bool;

pub open spec fn PowerCapChanged(s: S, domain_id: UInt32) -> bool;

pub open spec fn CaiChanged(s: S, domain_id: UInt32) -> bool;

pub open spec fn TransitionCompleted(s: S, domain_id: UInt32) -> bool;

pub open spec fn DomainPowerCap(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn DomainCai(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn DomainSupportsCpc(s: S, domain_id: UInt32) -> bool;

} // verus!

use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type AgentId = u32;
pub type RmiStatusCode = u64;
pub type NotificationType = u32;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        self is Ok
    }

    pub open spec fn is_Err(self) -> bool {
        self is Err
    }
}

pub struct S {
    pub dummy: u64,
}

pub const RMI_SUCCESS: RmiStatusCode = 0;
pub const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub const POWERCAP_CAP_CHANGED: NotificationType = 0x5;

pub uninterp spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub uninterp spec fn IsRegisteredForPowerCapNotification(s: S, recipient: AgentId, domain_id: UInt32) -> bool;

pub uninterp spec fn PowerCapChanged(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn CaiChanged(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn TransitionCompleted(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn NotificationSent(s: S, recipient: AgentId, notification: NotificationType) -> bool;

pub uninterp spec fn DomainPowerCap(s: S, domain_id: UInt32) -> UInt32;

pub uninterp spec fn DomainCai(s: S, domain_id: UInt32) -> UInt32;

pub uninterp spec fn DomainSupportsCpc(s: S, domain_id: UInt32) -> bool;

} // verus!

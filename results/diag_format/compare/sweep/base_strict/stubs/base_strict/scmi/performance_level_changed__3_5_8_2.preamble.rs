use vstd::prelude::*;

verus! {

#[is_variant]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub enum RmiStatusCode {
    Success,
    ErrorInput,
    ErrorRealm,
    ErrorRec,
    ErrorRtt,
    ErrorNotSupported,
    ErrorDenied,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn IsSubscribedToPerfLevelNotifications(s: S, recipient_agent: u32, domain_id: u32) -> bool;

pub open spec fn PerformanceLevelChanged(s: S, domain_id: u32) -> bool;

pub open spec fn PerformanceLevel(s: S, domain_id: u32) -> u32;

pub open spec fn PerfLevelChangeInitiator(s: S, domain_id: u32) -> u32;

} // verus!

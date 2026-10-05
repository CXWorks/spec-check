use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(&self) -> bool {
        self is Ok
    }

    pub open spec fn is_Err(&self) -> bool {
        self is Err
    }
}

pub struct S {
    pub registered: Map<(UInt32, UInt32), bool>,
    pub notified: Map<UInt32, bool>,
    pub mai_changed: Map<UInt32, bool>,
    pub average_power: Map<UInt32, UInt32>,
    pub lower_threshold: Map<(UInt32, UInt32), UInt32>,
    pub upper_threshold: Map<(UInt32, UInt32), UInt32>,
    pub domain_mai: Map<UInt32, UInt32>,
}

pub open spec fn IsRegisteredForMeasurementsChangeNotification(s: S, agent: UInt32, domain_id: UInt32) -> bool;

pub open spec fn NotificationSentTo(s: S, agent: UInt32) -> bool;

pub open spec fn MaiChanged(s: S, domain_id: UInt32) -> bool;

pub open spec fn AveragePowerOverMai(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn LowerPowerThreshold(s: S, agent: UInt32, domain_id: UInt32) -> UInt32;

pub open spec fn UpperPowerThreshold(s: S, agent: UInt32, domain_id: UInt32) -> UInt32;

pub open spec fn DomainMai(s: S, domain_id: UInt32) -> UInt32;

} // verus!

use vstd::prelude::*;
verus! {

pub enum RmiStatusCode {
    Success,
    ErrorInput,
    ErrorRealm,
    ErrorRec,
    ErrorRtt,
    ErrorDevice,
    ErrorNotSupported,
}

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
    pub dummy: int,
}

pub open spec fn IsRegisteredForMeasurementsChangedNotification(s: S, agent: u32, domain_id: u32) -> bool;

pub open spec fn MaiChanged(s: S, domain_id: u32) -> bool;

pub open spec fn AveragePowerBelowLowerThreshold(s: S, domain_id: u32) -> bool;

pub open spec fn AveragePowerAboveHigherThreshold(s: S, domain_id: u32) -> bool;

pub open spec fn AveragePowerAtThresholdBreach(s: S, domain_id: u32) -> u32;

pub open spec fn AveragePowerAtMaiChange(s: S, domain_id: u32) -> u32;

pub open spec fn PowerCapMai(s: S, domain_id: u32) -> u32;

} // verus!

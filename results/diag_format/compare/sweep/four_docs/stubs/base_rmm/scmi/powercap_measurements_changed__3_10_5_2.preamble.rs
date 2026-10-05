use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub enum RmiStatusCode {
    Success,
    ErrorInput,
    ErrorNotSupported,
    ErrorDenied,
    ErrorNotFound,
}

pub struct S {
    pub dummy: u32,
}

pub open spec fn IsRegisteredForMeasurementsChangeNotification(agent: UInt32, domain_id: UInt32) -> bool;

pub open spec fn MaiChanged(domain_id: UInt32) -> bool;

pub open spec fn AveragePowerOverMai(domain_id: UInt32) -> UInt32;

pub open spec fn DomainMai(domain_id: UInt32) -> UInt32;

pub open spec fn LowerPowerThreshold(agent: UInt32, domain_id: UInt32) -> UInt32;

pub open spec fn UpperPowerThreshold(agent: UInt32, domain_id: UInt32) -> UInt32;

} // verus!

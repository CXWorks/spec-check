use vstd::prelude::*;

verus! {

pub type RmiStatusCode = u64;

pub struct S {
    pub state: int,
}

pub spec const NOT_FOUND: RmiStatusCode = 10;
pub spec const INVALID_PARAMETERS: RmiStatusCode = 11;

pub spec const domain_id: u64 = 1;
pub spec const capability: u64 = 2;
pub spec const qos_value: u64 = 3;

pub open spec fn IsValidPerformanceDomain(s: S, domain: u64) -> bool;

pub open spec fn IsValidQosCapability(s: S, domain: u64, cap: u64) -> bool;

pub open spec fn QosCapabilityTypeBitCount(cap: u64) -> nat;

pub open spec fn QosCapabilitySubtypeBitCount(cap: u64) -> nat;

pub open spec fn QosConfiguration(s: S, domain: u64, cap: u64) -> u64;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

} // verus!

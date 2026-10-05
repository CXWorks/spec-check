use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub num_domains: nat,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-4) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2) as i32;

#[allow(non_upper_case_globals)]
pub spec const domain_id: UInt32 = 0;
#[allow(non_upper_case_globals)]
pub spec const capability: UInt32 = 1;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidQosCapabilityOfDomain(s: S, domain_id: UInt32, capability: UInt32) -> bool;

pub open spec fn QosCapabilityTypeBitCount(capability: UInt32) -> nat;

pub open spec fn QosCapabilitySubtypeBitCount(capability: UInt32) -> nat;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn CurrentQosConfig(s: S, domain_id: UInt32, capability: UInt32) -> UInt32;

} // verus!

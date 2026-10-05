use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint8 = u8;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_FOUND: int32 = (-4int) as int32;
pub spec const INVALID_PARAMETERS: int32 = (-2int) as int32;

#[allow(non_upper_case_globals)]
pub spec const domain_id: uint32 = 7;
#[allow(non_upper_case_globals)]
pub spec const capability: uint32 = 11;

pub open spec fn IsValidPerformanceDomain(domain_id: uint32) -> bool;

pub open spec fn IsValidQosCapabilityOfDomain(domain_id: uint32, capability: uint32) -> bool;

pub open spec fn ResultEqual(result: int32, code: int32) -> bool;

pub open spec fn CountQosCapabilityTypeBits(capability: uint32) -> int;

pub open spec fn CountQosCapabilitySubtypeBits(capability: uint32) -> int;

pub open spec fn QosCapabilityFirstAttribute(domain_id: uint32, capability: uint32) -> uint32;

pub open spec fn QosCapabilityName(domain_id: uint32, capability: uint32) -> [uint8; 16];

} // verus!

use vstd::prelude::*;

verus! {

pub type uint8 = u8;
pub type uint32 = u32;
pub type int32 = i32;

pub type ErrorCode = i32;

pub spec const NOT_FOUND: ErrorCode = (-4int) as i32;
pub spec const INVALID_PARAMETERS: ErrorCode = (-2int) as i32;

pub struct S {
    pub num_domains: nat,
}

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: uint32) -> bool;

pub open spec fn IsValidQosCapabilityOfDomain(s: S, domain_id: uint32, capability: uint32) -> bool;

pub open spec fn CountQosCapabilityTypeBits(s: S, capability: uint32) -> int;

pub open spec fn CountQosCapabilitySubtypeBits(s: S, capability: uint32) -> int;

pub open spec fn QosCapabilityFirstAttribute(s: S, domain_id: uint32, capability: uint32) -> uint32;

pub open spec fn QosCapabilityName(s: S, domain_id: uint32, capability: uint32) -> [uint8; 16];

pub open spec fn ResultEqual(r: Result<(), ErrorCode>, status: int32) -> bool;

} // verus!

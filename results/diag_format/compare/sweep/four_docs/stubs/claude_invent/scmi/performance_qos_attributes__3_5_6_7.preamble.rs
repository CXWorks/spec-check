use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -3;
pub const INVALID_PARAMETERS: i32 = -2;

pub open spec fn QosCapabilityHasMultipleTypeOrSubtypeBits(capability: u32) -> bool;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: u32) -> bool;

pub open spec fn IsValidQosCapabilityOfDomain(s: S, domain_id: u32, capability: u32) -> bool;

pub open spec fn PerformanceQosAttribute1(s: S, domain_id: u32, capability: u32) -> u32;

pub open spec fn PerformanceQosCapabilityName(s: S, domain_id: u32, capability: u32) -> Seq<u8>;

} // verus!

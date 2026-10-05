use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -2;
pub const INVALID_PARAMETERS: i32 = -3;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: u32) -> bool;
pub open spec fn QosCapabilityTypeBitCount(capability: u32) -> nat;
pub open spec fn QosCapabilitySubtypeBitCount(capability: u32) -> nat;
pub open spec fn IsSupportedQosCapability(s: S, domain_id: u32, capability: u32) -> bool;
pub open spec fn PerformanceQosConfig(s: S, domain_id: u32, capability: u32) -> u32;

} // verus!

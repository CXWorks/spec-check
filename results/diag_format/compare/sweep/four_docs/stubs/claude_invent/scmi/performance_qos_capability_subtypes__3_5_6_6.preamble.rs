use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;

pub struct S {
    pub state_id: nat,
}

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidQosCapabilityType(s: S, domain_id: UInt32, capability_type: UInt32) -> bool;

pub open spec fn QosCapabilitySubtypesOf(s: S, domain_id: UInt32, capability_type: UInt32) -> UInt32;

} // verus!

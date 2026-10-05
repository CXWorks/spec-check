use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;
pub const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn IsValidPerfDomain(domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn BitCount(x: int) -> int;

pub open spec fn IsValidQosCapabilityType(domain_id: UInt32, capability_type: UInt32) -> bool;

pub open spec fn SupportedQosCapabilitySubtypes(domain_id: UInt32, capability_type: UInt32) -> UInt32;

} // verus!

use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn BitCount(x: UInt32) -> nat;

pub open spec fn IsValidQosCapabilityType(s: S, domain_id: UInt32, capability_type: UInt32) -> bool;

pub open spec fn SupportedQosCapabilitySubtypes(s: S, domain_id: UInt32, capability_type: UInt32) -> UInt32;

} // verus!

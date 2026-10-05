use vstd::prelude::*;
verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -3;
pub const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidQosCapability(s: S, domain_id: UInt32, capability: UInt32) -> bool;

pub open spec fn QosCapabilityTypeBitCount(s: S, capability: UInt32) -> int;

pub open spec fn QosCapabilitySubtypeBitCount(s: S, capability: UInt32) -> int;

pub open spec fn QosCapabilityFirstAttribute(s: S, domain_id: UInt32, capability: UInt32) -> UInt32;

pub open spec fn QosCapabilityNameEqual(s: S, name: [UInt8; 16], domain_id: UInt32, capability: UInt32) -> bool;

pub open spec fn IsNullTerminatedAscii(s: S, name: [UInt8; 16], len: int) -> bool;

} // verus!

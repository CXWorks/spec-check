use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -3;
pub spec const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn ResultEqual(status: Int32, expected: Int32) -> bool;

pub open spec fn IsValidPerformanceDomain(domain_id: UInt32) -> bool;

pub open spec fn IsValidQosCapability(domain_id: UInt32, capability: UInt32) -> bool;

pub open spec fn QosCapabilityTypeBitCount(capability: UInt32) -> int;

pub open spec fn QosCapabilitySubtypeBitCount(capability: UInt32) -> int;

pub open spec fn QosCapabilityFirstAttribute(domain_id: UInt32, capability: UInt32) -> UInt32;

pub open spec fn QosCapabilityNameEqual(name: [UInt8; 16], domain_id: UInt32, capability: UInt32) -> bool;

pub open spec fn IsNullTerminatedAscii(name: [UInt8; 16], len: int) -> bool;

} // verus!

use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0i32;
pub spec const NOT_FOUND: Int32 = -1i32;
pub spec const INVALID_PARAMETERS: Int32 = -2i32;

pub open spec fn IsValidPerfDomain(domain_id: UInt32) -> bool;

pub open spec fn IsValidQosCapabilityType(domain_id: UInt32, capability_type: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn PopCount(x: UInt32) -> nat;

pub open spec fn Bits(x: UInt32, hi: nat, lo: nat) -> UInt32;

pub open spec fn SupportedOemQosSubtypes(domain_id: UInt32, capability_type: UInt32) -> UInt32;

pub open spec fn SupportedArchitectedQosSubtypes(domain_id: UInt32, capability_type: UInt32) -> UInt32;

} // verus!

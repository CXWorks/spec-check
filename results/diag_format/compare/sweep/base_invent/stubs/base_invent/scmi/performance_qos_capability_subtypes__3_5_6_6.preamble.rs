use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;

pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool;

pub open spec fn CapabilityTypeIsValid(s: S, domain_id: uint32, capability_type: uint32) -> bool;

pub open spec fn CountSetBits(x: uint32) -> int;

pub open spec fn ResultEqual(result: int32, expected: int32) -> bool;

} // verus!

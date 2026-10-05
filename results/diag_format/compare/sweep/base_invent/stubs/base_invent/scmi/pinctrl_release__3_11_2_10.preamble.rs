use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub pins: Map<uint32, bool>,
    pub groups: Map<uint32, bool>,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;

pub spec const identifier: uint32 = 7;
pub spec const flags: uint32 = 3;
pub spec const selector: uint32 = 5;

pub open spec fn PinOrGroupExists(s: S, id: uint32) -> bool;

} // verus!

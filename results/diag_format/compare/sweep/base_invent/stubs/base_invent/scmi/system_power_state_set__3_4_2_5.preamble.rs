use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub flags: u32,
    pub system_state: u32,
    pub reset_type: u32,
}

pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -3;
pub const NOT_SUPPORTED: int32 = -1;

pub open spec fn flags_bits(s: S) -> u32;

pub open spec fn system_state_bits(s: S) -> u32;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

} // verus!

use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub flags: uint32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -3;
pub const NOT_FOUND: int32 = -4;

pub open spec fn flags_not_zero(flags: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

} // verus!

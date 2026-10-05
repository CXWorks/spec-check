use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub flags: uint32,
    pub domain_id: int32,
    pub reset_state: int32,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_SUPPORTED: int32 = (-1) as int32;
pub spec const INVALID_PARAMETERS: int32 = (-2) as int32;
pub spec const DENIED: int32 = (-3) as int32;
pub spec const NOT_FOUND: int32 = (-4) as int32;
pub spec const GENERIC_ERROR: int32 = (-8) as int32;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

} // verus!

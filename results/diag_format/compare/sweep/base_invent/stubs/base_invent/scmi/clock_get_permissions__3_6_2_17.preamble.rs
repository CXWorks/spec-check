use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const NOT_FOUND: int32 = -7;

pub spec const clock_id: uint32 = 0;

pub struct S {
    pub dummy: int,
}

impl S {
    pub open spec fn clock_permissions(self, id: uint32) -> uint32;
}

pub open spec fn clock_id_is_invalid(s: S, id: uint32) -> bool;

pub open spec fn request_not_supported(s: S) -> bool;

} // verus!

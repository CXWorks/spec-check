use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0i32;
pub spec const NOT_FOUND: int32 = -4i32;

pub spec const message_id: uint32 = 0u32;

} // verus!

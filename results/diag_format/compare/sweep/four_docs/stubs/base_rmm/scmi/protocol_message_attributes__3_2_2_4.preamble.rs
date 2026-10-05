use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_FOUND: int32 = (-1int) as int32;

pub open spec fn IsMessageImplemented(message_id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

} // verus!

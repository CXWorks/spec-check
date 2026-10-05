use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0i32;
pub const NOT_FOUND: i32 = -4i32;

pub open spec fn ScmiMessageImplemented(s: S, protocol_id: u32, message_id: u32) -> bool;

} // verus!

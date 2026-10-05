use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub implemented_messages: Set<u32>,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;

} // verus!

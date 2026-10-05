use vstd::prelude::*;
verus! {

pub struct S {
    pub implemented_messages: Set<u32>,
    pub agent_count: u32,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub open spec fn IsMessageImplemented(s: S, message_id: u32) -> bool;

} // verus!

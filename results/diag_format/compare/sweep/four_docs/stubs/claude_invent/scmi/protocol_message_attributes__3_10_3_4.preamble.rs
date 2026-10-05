use vstd::prelude::*;

verus! {

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsMessageImplemented(s: S, message_id: u32) -> bool;

pub open spec fn IsMessageAvailable(s: S, message_id: u32) -> bool;

pub open spec fn HasDedicatedFastChannel(s: S, message_id: u32) -> bool;

} // verus!

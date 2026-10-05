use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub open spec fn IsMessageImplementedAndAvailable(s: S, protocol_id: UInt32, message_id: UInt32) -> bool;

} // verus!

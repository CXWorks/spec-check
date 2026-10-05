use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub clock_count: nat,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn IsClockProtocolMessageImplemented(s: S, message_id: UInt32) -> bool;

} // verus!

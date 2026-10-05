use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub clock_rate: [UInt32; 2],
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -6;

pub const clock_id: UInt32 = 0;

pub open spec fn ClockExists(s: S, id: UInt32) -> bool;

} // verus!

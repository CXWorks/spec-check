use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct Clock {
    pub rate: UInt32,
}

pub struct S {
    pub clocks: Seq<Clock>,
}

pub const SUCCESS: Int32 = 0;
pub const DENIED: Int32 = -3;

pub open spec fn ClockHasOtherUsers(s: S, clock_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn ClockAt(s: S, clock_id: UInt32) -> Clock;

} // verus!

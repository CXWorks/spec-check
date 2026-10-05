use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const DENIED: Int32 = (-3int) as i32;

pub open spec fn ClockHasOtherUsers(s: S, clock_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn ClockRate(s: S, clock_id: UInt32) -> int;

} // verus!

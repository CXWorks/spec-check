use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub clocks: Seq<u32>,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -4i32;

#[allow(non_upper_case_globals)]
pub spec const clock_id: UInt32 = 0;

pub open spec fn ClockExists(s: S, id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn ClockIsDisabled(s: S, id: UInt32) -> bool;

pub open spec fn ClockCurrentRate(s: S, id: UInt32) -> int;

pub open spec fn ClockRateOnReenable(s: S, id: UInt32) -> int;

} // verus!

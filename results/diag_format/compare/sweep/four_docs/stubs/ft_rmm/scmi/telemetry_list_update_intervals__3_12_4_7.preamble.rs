use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct Flags {
    pub value: u32,
}

pub struct Flags1 {
    pub count: u32,
    pub format: u32,
    pub remaining: u32,
}

pub struct S {
    pub num_update_intervals: u32,
}

pub const SUCCESS: Int32 = 0;
pub const OUT_OF_RANGE: Int32 = 1;

pub open spec fn IsValidUpdateIntervalIndex(s: S, index: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsNumericAscending(s: S, intervals: [UInt32; 4], count: u32) -> bool;

} // verus!

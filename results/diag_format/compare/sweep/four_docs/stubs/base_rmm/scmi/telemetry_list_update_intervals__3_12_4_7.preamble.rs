use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const OUT_OF_RANGE: Int32 = 1;

pub open spec fn IsValidUpdateIntervalIndex(index: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsNumericAscending(intervals: &[UInt32], count: int) -> bool;

} // verus!

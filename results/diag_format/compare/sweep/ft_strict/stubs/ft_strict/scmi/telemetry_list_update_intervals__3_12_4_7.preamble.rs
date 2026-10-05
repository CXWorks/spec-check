use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;

pub const OUT_OF_RANGE: Int32 = 1;

pub const RSI_SUCCESS: Int32 = 2;

pub spec const result: Int32 = 3;

pub open spec fn IsValidUpdateIntervalIndex(s: S, group_identifier: UInt32, flags: UInt32, index: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn Bits(value: UInt32, hi: UInt32, lo: UInt32) -> UInt32;

pub open spec fn Bit(value: UInt32, pos: UInt32) -> UInt32;

pub open spec fn ArrayLength(arr: [UInt32; 4]) -> UInt32;

pub open spec fn ArrayEntry(arr: [UInt32; 4], i: UInt32) -> UInt32;

pub open spec fn LowestSupportedUpdateInterval(s: S, group_identifier: UInt32, flags: UInt32) -> UInt32;

pub open spec fn HighestSupportedUpdateInterval(s: S, group_identifier: UInt32, flags: UInt32) -> UInt32;

pub open spec fn UpdateIntervalStepSize(s: S, group_identifier: UInt32, flags: UInt32) -> UInt32;

pub open spec fn SupportedUpdateIntervalAt(s: S, group_identifier: UInt32, flags: UInt32, idx: int) -> UInt32;

pub open spec fn NumSupportedUpdateIntervals(s: S, group_identifier: UInt32, flags: UInt32) -> UInt32;

pub open spec fn IntervalValue(v: UInt32) -> int;

} // verus!

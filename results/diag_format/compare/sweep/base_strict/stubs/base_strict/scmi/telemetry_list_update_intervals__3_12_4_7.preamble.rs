use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct Array<T> {
    pub data: Seq<T>,
}

pub struct S {
    pub group_identifier: UInt32,
    pub flags: UInt32,
    pub index: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const OUT_OF_RANGE: Int32 = 1;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn group_identifier(s: S) -> UInt32;

pub open spec fn flags(s: S) -> UInt32;

pub open spec fn index(s: S) -> UInt32;

pub open spec fn IsValidUpdateIntervalIndex(s: S, group_id: UInt32, flags: UInt32, index: UInt32) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn Bit(value: UInt32, pos: int) -> UInt32;

pub open spec fn ArrayLength<A>(a: Array<A>) -> UInt32;

pub open spec fn ArrayEntry<A, I>(a: Array<A>, i: I) -> A;

pub open spec fn LowestSupportedUpdateInterval(s: S, group_id: UInt32, flags: UInt32) -> UInt32;

pub open spec fn HighestSupportedUpdateInterval(s: S, group_id: UInt32, flags: UInt32) -> UInt32;

pub open spec fn UpdateIntervalStepSize(s: S, group_id: UInt32, flags: UInt32) -> UInt32;

pub open spec fn SupportedUpdateIntervalAt(s: S, group_id: UInt32, flags: UInt32, idx: int) -> UInt32;

pub open spec fn NumSupportedUpdateIntervals(s: S, group_id: UInt32, flags: UInt32) -> int;

pub open spec fn IntervalValue(interval: UInt32) -> int;

} // verus!

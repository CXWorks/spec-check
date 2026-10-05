use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub struct S {
    pub enabled_list_index: UInt32,
    pub enabled_list_flags: UInt32,
}

pub const SUCCESS: Int32 = 0;

pub const OUT_OF_RANGE: Int32 = -2;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn IsValidEnabledListIndex(index: UInt32, kind: UInt32) -> bool;

pub open spec fn ArrayLength<A>(array: A) -> UInt32;

pub open spec fn ArrayEntryWord<A>(array: A, i: UInt32, word: int) -> UInt32;

pub open spec fn RemainingEnabledElements(index: UInt32, kind: UInt32, count: UInt32) -> UInt32;

pub open spec fn IsEnabledElement(element: UInt32, kind: UInt32) -> bool;

pub open spec fn IsEnabledWithTimestamps(element: UInt32, kind: UInt32) -> bool;

} // verus!

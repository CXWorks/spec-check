use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;

pub struct S {
    pub enabled_des: Seq<UInt32>,
    pub enabled_event_groups: Seq<UInt32>,
}

pub struct Entry {
    pub mode: u8,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const OUT_OF_RANGE: Int32 = 1;

pub spec const index: UInt16 = 0;
pub spec const selector: UInt16 = 0;

pub open spec fn IsValidEnabledListIndex(idx: UInt16, sel: UInt16) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Length(array: [UInt32]) -> UInt16;

pub open spec fn AllEntriesAreEnabledDes(array: [UInt32]) -> bool;

pub open spec fn AllEntriesAreEnabledEventGroups(array: [UInt32]) -> bool;

pub open spec fn ForAll(array: [UInt32], f: spec_fn(Entry) -> bool) -> bool;

} // verus!

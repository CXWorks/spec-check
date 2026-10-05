use vstd::prelude::*;

verus! {

pub type UInt1 = u8;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct EnabledListEntry {
    pub id: u32,
    pub mode: u8,
}

pub struct S {
    pub enabled_des: Seq<u32>,
    pub enabled_event_groups: Seq<u32>,
    pub num_des: u32,
    pub num_event_groups: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const OUT_OF_RANGE: Int32 = 1;

pub open spec fn IsValidEnabledListIndex(s: S, index: UInt32, selector: UInt1) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Length(array: [EnabledListEntry; 16]) -> UInt16;

pub open spec fn AllEntriesAreEnabledDes(array: [EnabledListEntry; 16]) -> bool;

pub open spec fn AllEntriesAreEnabledEventGroups(array: [EnabledListEntry; 16]) -> bool;

pub open spec fn ForAll(array: [EnabledListEntry; 16], pred: spec_fn(EnabledListEntry) -> bool) -> bool;

} // verus!

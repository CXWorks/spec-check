use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct DE_DESC {
    pub de_id: UInt32,
    pub de_size: UInt32,
    pub de_flags: UInt32,
}

pub struct S {
    pub de_count: UInt32,
    pub de_cursor: UInt32,
}

pub open spec fn NumRemainingDeDescriptors(desc_index: UInt32, num_returned: UInt16) -> UInt16;

pub open spec fn DeDescriptorAt(desc_index: UInt32) -> DE_DESC;

pub open spec fn IsDeDescFormat(d: DE_DESC) -> bool;

} // verus!

use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;

pub struct DE_DESC {
    pub de_id: UInt32,
    pub de_attributes_1: UInt32,
    pub de_attributes_2: UInt32,
    pub de_attributes_3: UInt32,
    pub name: Seq<u8>,
}

pub struct S {
    pub de_count: UInt32,
}

pub spec const SUCCESS: Int32 = 0;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn RemainingDeDescriptors(desc_index: UInt32, num: int) -> int;

pub open spec fn DeDescriptorAt(index: int) -> DE_DESC;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

pub open spec fn DeDescHasLineTsRate(d: DE_DESC) -> bool;

pub open spec fn DeDescHasFastChannelFields(d: DE_DESC) -> bool;

pub open spec fn FastChannelSizeSufficient(d: DE_DESC) -> bool;

pub open spec fn DeDescHasName(d: DE_DESC) -> bool;

pub open spec fn IsNullTerminatedUtf8(s: Seq<u8>, max_len: int) -> bool;

pub open spec fn DeDataSpaceSufficient(d: DE_DESC) -> bool;

} // verus!

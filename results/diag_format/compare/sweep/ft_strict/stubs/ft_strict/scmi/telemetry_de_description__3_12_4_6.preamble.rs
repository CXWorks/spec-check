use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub const SUCCESS: Int32 = 0;

pub struct DE_DESC {
    pub de_id: UInt32,
    pub de_data_size: UInt32,
    pub de_attributes_1: UInt32,
    pub de_attributes_2: UInt32,
    pub de_attributes_3: UInt32,
    pub name: [u8; 16],
}

pub struct S {
    pub num_de: UInt32,
}

pub open spec fn Bits<T>(x: T, hi: int, lo: int) -> UInt32;

pub open spec fn RemainingDeDescriptors(s: S, desc_index: UInt32, n: UInt32) -> UInt32;

pub open spec fn DeDescriptorAt(s: S, index: int) -> DE_DESC;

pub open spec fn DeDescHasLineTsRate(s: S, d: DE_DESC) -> bool;

pub open spec fn DeDescHasFastChannelFields(s: S, d: DE_DESC) -> bool;

pub open spec fn FastChannelSizeSufficient(s: S, d: DE_DESC) -> bool;

pub open spec fn DeDescHasName(s: S, d: DE_DESC) -> bool;

pub open spec fn IsNullTerminatedUtf8(s: S, name: [u8; 16], max_len: int) -> bool;

pub open spec fn DeDataSpaceSufficient(s: S, d: DE_DESC) -> bool;

} // verus!

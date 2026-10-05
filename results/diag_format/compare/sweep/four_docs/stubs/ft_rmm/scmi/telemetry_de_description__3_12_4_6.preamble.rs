use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub struct DE_DESC {
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn DeDescriptorAt(s: S, desc_index: UInt32) -> DE_DESC;

pub open spec fn NumRemainingDeDescriptors(s: S, desc_index: UInt32, num_returned: UInt16) -> UInt16;

pub open spec fn IsDeDescFormat(d: DE_DESC) -> bool;

} // verus!

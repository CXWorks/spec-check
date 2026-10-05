use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub struct S {
    pub dummy: u64,
}

pub open spec fn NumPowerCappingDomains() -> u32;

pub open spec fn u32_bit_slice(x: u32, r: core::ops::Range<int>) -> u32;

pub trait BitSliceIndex {
    spec fn spec_index(self, r: core::ops::Range<int>) -> u32;
}

impl BitSliceIndex for u32 {
    open spec fn spec_index(self, r: core::ops::Range<int>) -> u32 {
        u32_bit_slice(self, r)
    }
}

} // verus!

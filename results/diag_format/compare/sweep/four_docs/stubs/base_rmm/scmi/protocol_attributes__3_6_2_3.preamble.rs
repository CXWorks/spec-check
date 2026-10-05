use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub open spec fn MaxPendingAsyncClockRateChanges() -> uint32;

pub open spec fn NumClocks() -> uint32;

pub open spec fn u32_bit_slice(x: u32, r: core::ops::Range<int>) -> int;

pub trait BitSliceIndex {
    spec fn spec_index(self, r: core::ops::Range<int>) -> int;
}

impl BitSliceIndex for u32 {
    open spec fn spec_index(self, r: core::ops::Range<int>) -> int {
        u32_bit_slice(self, r)
    }
}

} // verus!

use vstd::prelude::*;

verus! {

pub type uint32 = u32;

pub struct S {
    pub dummy: u32,
}

pub open spec fn MaxPendingAsyncClockRateChanges() -> uint32;

pub open spec fn NumClocks() -> uint32;

} // verus!

use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;

pub open spec fn ResultEqual(status: int32, expected: int32) -> bool;

pub open spec fn Bits(value: uint32, hi: int, lo: int) -> int;

pub open spec fn MaxPendingAsyncClockRateChanges() -> int;

pub open spec fn NumClocks() -> int;

} // verus!

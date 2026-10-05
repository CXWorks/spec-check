use vstd::prelude::*;
verus! {

pub type uint32 = u32;

pub struct S {
    pub num_clocks: int,
    pub max_pending_async_clock_rate_changes: int,
}

pub spec const SUCCESS: int = 0;

pub open spec fn ResultEqual(status: int, expected: int) -> bool;

pub open spec fn Bits(value: uint32, high: int, low: int) -> int;

pub open spec fn MaxPendingAsyncClockRateChanges() -> int;

pub open spec fn NumClocks() -> int;

} // verus!

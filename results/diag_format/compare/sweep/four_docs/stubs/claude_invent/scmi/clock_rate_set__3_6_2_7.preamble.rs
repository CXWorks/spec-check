use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type ScmiStatusCode = i32;

pub const SUCCESS: ScmiStatusCode = 0;
pub const NOT_SUPPORTED: ScmiStatusCode = -1;
pub const INVALID_PARAMETERS: ScmiStatusCode = -2;
pub const DENIED: ScmiStatusCode = -3;
pub const NOT_FOUND: ScmiStatusCode = -4;
pub const OUT_OF_RANGE: ScmiStatusCode = -5;
pub const BUSY: ScmiStatusCode = -6;

pub struct S {
    pub dummy: int,
}

pub open spec fn ClockExists(s: S, clock_id: int) -> bool;

pub open spec fn ClockRateSupported(s: S, clock_id: int, rate: int) -> bool;

pub open spec fn ClockAsyncRateChangesPendingFull(s: S) -> bool;

pub open spec fn ClockRateSetDeniedByDependencies(s: S, clock_id: int) -> bool;

pub open spec fn ClockRate(s: S, clock_id: int) -> int;

pub open spec fn ClockRoundedRate(s: S, clock_id: int, rate: int, round_auto: bool, round_up: bool) -> int;

pub open spec fn ClockRateChangeQueued(s: S, clock_id: int, rate: int) -> bool;

pub open spec fn ClockRateSetDelayedResponsePending(s: S, clock_id: int) -> bool;

} // verus!

use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub pending_async: nat,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const BUSY: Int32 = -6;

pub const CLOCK_RATE_SET_COMPLETE: UInt32 = 5;

pub const clock_id: UInt32 = 11;
pub const flags: UInt32 = 13;
pub const rate: UInt64 = 17;

pub open spec fn ClockExists(s: S, id: UInt32) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn RequestedRate(r: UInt64) -> UInt64;
pub open spec fn ClockSupportsRate(s: S, id: UInt32, r: UInt64) -> bool;
pub open spec fn IsValidClockRateSetFlags(s: S, f: UInt32) -> bool;
pub open spec fn Bits(s: S, f: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn PendingAsyncClockRateChanges(s: S) -> nat;
pub open spec fn MaxPendingAsyncClockRateChanges() -> nat;
pub open spec fn ClockRateChangeBlockedByDependencies(s: S, id: UInt32) -> bool;
pub open spec fn ClockIsEnabled(s: S, id: UInt32) -> bool;
pub open spec fn ClockRate(s: S, id: UInt32) -> UInt64;
pub open spec fn SelectPhysicalRate(s: S, id: UInt32, r: UInt64, round_mode: UInt32) -> UInt64;
pub open spec fn RateTakesEffectOnReEnable(s: S, id: UInt32, r: UInt64) -> bool;
pub open spec fn AsyncClockRateChangeQueued(s: S, id: UInt32, r: UInt64) -> bool;
pub open spec fn DelayedResponseSent(msg: UInt32, id: UInt32) -> bool;

} // verus!

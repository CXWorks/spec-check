use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;
pub const BUSY: Int32 = -6;
pub const result: Int32 = -100;

pub const CLOCK_RATE_SET_COMPLETE: UInt32 = 0x5;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn ClockExists(s: S, clock_id: UInt32) -> bool;

pub open spec fn RequestedRate(rate: [UInt32; 2]) -> UInt64;

pub open spec fn ClockSupportsRate(s: S, clock_id: UInt32, rate: UInt64) -> bool;

pub open spec fn IsValidClockRateSetFlags(s: S, flags: UInt32) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

pub open spec fn PendingAsyncClockRateChanges(s: S) -> nat;

pub open spec fn MaxPendingAsyncClockRateChanges(s: S) -> nat;

pub open spec fn ClockRateChangeBlockedByDependencies(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockIsEnabled(s: S, clock_id: UInt32) -> bool;

pub open spec fn ClockRate(s: S, clock_id: UInt32) -> UInt64;

pub open spec fn SelectPhysicalRate(s: S, clock_id: UInt32, rate: UInt64, round_mode: int) -> UInt64;

pub open spec fn RateTakesEffectOnReEnable(s: S, clock_id: UInt32, rate: UInt64) -> bool;

pub open spec fn AsyncClockRateChangeQueued(s: S, clock_id: UInt32, rate: UInt64) -> bool;

pub open spec fn DelayedResponseSent(s: S, message_id: UInt32, clock_id: UInt32) -> bool;

} // verus!

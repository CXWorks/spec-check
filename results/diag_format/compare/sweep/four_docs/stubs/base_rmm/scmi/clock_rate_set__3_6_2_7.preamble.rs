use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -3;
pub const NOT_FOUND: int32 = -4;
pub const BUSY: int32 = -6;

pub const CLOCK_RATE_SET: u32 = 5;
pub const CLOCK_RATE_SET_COMPLETE: u32 = 6;

pub open spec fn clock_id_val() -> u32;
pub open spec fn rate_val() -> u64;
pub open spec fn flags_val() -> Seq<int>;

pub spec const clock_id: u32 = clock_id_val();
pub spec const rate: u64 = rate_val();
pub spec const flags: Seq<int> = flags_val();

pub open spec fn ClockExists(s: S, clock_id: u32) -> bool;
pub open spec fn ClockSupportsRate(s: S, clock_id: u32, rate: u64) -> bool;
pub open spec fn IsValidClockRateSetFlags(s: S, flags: Seq<int>) -> bool;
pub open spec fn PendingAsyncClockRateChanges(s: S) -> nat;
pub open spec fn MaxPendingAsyncClockRateChanges() -> nat;
pub open spec fn ClockRateBlockedByDependencies(s: S, clock_id: u32) -> bool;
pub open spec fn ResultEqual(result: int32, code: int32) -> bool;
pub open spec fn ClockRate(s: S, clock_id: u32) -> u64;
pub open spec fn RoundRate(s: S, clock_id: u32, rate: u64, round_mode: Seq<int>) -> u64;
pub open spec fn ClockEnabled(s: S, clock_id: u32) -> bool;
pub open spec fn CommandQueued(cmd: u32, clock_id: u32, rate: u64) -> bool;

} // verus!

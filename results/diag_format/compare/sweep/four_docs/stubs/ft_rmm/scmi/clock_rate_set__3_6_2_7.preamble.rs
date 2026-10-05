use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;
pub type RmiStatusCode = i32;
pub type CommandId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: RmiStatusCode = 0;
pub const NOT_FOUND: RmiStatusCode = -1;
pub const INVALID_PARAMETERS: RmiStatusCode = -2;
pub const BUSY: RmiStatusCode = -3;
pub const DENIED: RmiStatusCode = -4;

pub const CLOCK_RATE_SET: CommandId = 5;
pub const CLOCK_RATE_SET_COMPLETE: CommandId = 6;

pub open spec fn ClockExists(s: S, clock_id: uint32) -> bool;
pub open spec fn ResultEqual(result: Result<int32, RmiStatusCode>, code: RmiStatusCode) -> bool;
pub open spec fn ClockSupportsRate(s: S, clock_id: uint32, rate: [uint32; 2]) -> bool;
pub open spec fn IsValidClockRateSetFlags(s: S, flags: uint32) -> bool;
pub open spec fn PendingAsyncClockRateChanges(s: S) -> nat;
pub open spec fn MaxPendingAsyncClockRateChanges(s: S) -> nat;
pub open spec fn ClockRateBlockedByDependencies(s: S, clock_id: uint32) -> bool;
pub open spec fn ClockRate(s: S, clock_id: uint32) -> u64;
pub open spec fn RoundRate(s: S, clock_id: uint32, rate: u64) -> u64;
pub open spec fn ClockEnabled(s: S, clock_id: uint32) -> bool;
pub open spec fn CommandQueued(s: S, cmd: CommandId, clock_id: uint32, rate: [uint32; 2]) -> bool;

} // verus!

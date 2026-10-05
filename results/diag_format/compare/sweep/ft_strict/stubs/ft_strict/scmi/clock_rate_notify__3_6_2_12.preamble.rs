use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type RmiStatusCode = u32;

pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: RmiStatusCode = 0;
pub const NOT_FOUND: RmiStatusCode = 1;
pub const INVALID_PARAMETERS: RmiStatusCode = 2;

pub open spec fn IsValidClockDevice(s: S, clock_id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

pub open spec fn ClockRateNotifyEnabled(s: S, agent: AgentId, clock_id: UInt32) -> bool;

pub open spec fn CallingAgent() -> AgentId;

} // verus!

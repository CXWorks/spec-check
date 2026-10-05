use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type AgentId = u32;

pub struct S {
    pub clock_id: UInt32,
    pub notify_enable: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_FOUND: Int32 = -4;

pub open spec fn clock_id(s: S) -> UInt32;

pub open spec fn notify_enable(s: S) -> UInt32;

pub open spec fn IsValidClockDevice(id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(v: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(v: UInt32, hi: int, lo: int) -> int;

pub open spec fn CallingAgent() -> AgentId;

pub open spec fn ClockRateNotifyEnabled(agent: AgentId, id: UInt32) -> bool;

} // verus!

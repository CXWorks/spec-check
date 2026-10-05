use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub enum RsiCommandReturnCode {
    Success,
    NotFound,
    InvalidParameters,
    Denied,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;

pub spec const RSI_SUCCESS: Result<Int32, RsiCommandReturnCode> = Ok(0i32);

pub spec const calling_agent: AgentId = 0;

pub open spec fn IsValidClockDevice(s: S, clock_id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<Int32, RsiCommandReturnCode>, code: Int32) -> bool;

pub open spec fn ClockRateNotifyEnabled(s: S, agent: AgentId, clock_id: UInt32) -> bool;

} // verus!

use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub system_power_state: UInt32,
    pub agent_count: UInt32,
}

pub spec const OK: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2) as i32;
pub spec const DENIED: Int32 = (-3) as i32;

pub spec const caller: AgentId = 0;
pub spec const result: Int32 = (-100) as i32;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsValidSystemPowerState(s: S, system_state: UInt32) -> bool;

pub open spec fn IsSystemPowerStateSupportedForAgent(s: S, system_state: UInt32, agent: AgentId) -> bool;

pub open spec fn OtherApplicationProcessorsRunningOrIdle(s: S, agent: AgentId) -> bool;

pub open spec fn SystemPowerState(s: S) -> UInt32;

} // verus!

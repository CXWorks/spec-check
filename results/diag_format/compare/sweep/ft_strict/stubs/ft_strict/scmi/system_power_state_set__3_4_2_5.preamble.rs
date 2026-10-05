use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type ApplicationProcessor = u64;
pub type Agent = u32;

pub struct S {
    pub dummy: u64,
}

pub spec const RSI_SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const DENIED: Int32 = -3;

pub spec const result: Int32 = 1;

pub spec const caller_agent: Agent = 0;
pub spec const caller_cpu: ApplicationProcessor = 0;

pub open spec fn IsValidSystemPowerState(s: S, system_state: UInt32) -> bool;

pub open spec fn IsSystemPowerStateSupportedForAgent(s: S, system_state: UInt32, agent: Agent) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn CpuIsRunning(cpu: ApplicationProcessor) -> bool;

pub open spec fn CpuIsIdle(cpu: ApplicationProcessor) -> bool;

pub open spec fn SystemPowerStateRequested(s: S, system_state: UInt32) -> bool;

pub open spec fn GracefulSystemPowerStateRequested(s: S, system_state: UInt32) -> bool;

pub open spec fn ForcefulSystemPowerStateRequested(s: S, system_state: UInt32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

} // verus!

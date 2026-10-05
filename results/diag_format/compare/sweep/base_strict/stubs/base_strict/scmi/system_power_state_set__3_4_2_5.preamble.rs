use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub type ApplicationProcessor = u64;

pub type Agent = u32;

pub struct SystemPowerStateSetArgs {
    pub system_state: UInt32,
    pub flags: UInt32,
    pub caller_agent: Agent,
    pub caller_cpu: ApplicationProcessor,
}

pub struct S {
    pub system_power_state_set: SystemPowerStateSetArgs,
}

pub const SUCCESS: Int32 = 0;

pub const NOT_SUPPORTED: Int32 = -1;

pub const INVALID_PARAMETERS: Int32 = -2;

pub const DENIED: Int32 = -3;

pub open spec fn IsValidSystemPowerState(state: UInt32) -> bool;

pub open spec fn IsSystemPowerStateSupportedForAgent(state: UInt32, agent: Agent) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn CpuIsRunning(cpu: ApplicationProcessor) -> bool;

pub open spec fn CpuIsIdle(cpu: ApplicationProcessor) -> bool;

pub open spec fn SystemPowerStateRequested(state: UInt32) -> bool;

pub open spec fn GracefulSystemPowerStateRequested(state: UInt32) -> bool;

pub open spec fn ForcefulSystemPowerStateRequested(state: UInt32) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;

} // verus!

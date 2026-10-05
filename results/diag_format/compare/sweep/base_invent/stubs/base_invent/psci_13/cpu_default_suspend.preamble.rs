use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const PSCI_ERROR_NOT_SUPPORTED: PsciReturnCode = -1;
pub const PSCI_ERROR_INVALID_PARAMETERS: PsciReturnCode = -2;
pub const PSCI_ERROR_DENIED: PsciReturnCode = -3;
pub const PSCI_ERROR_ALREADY_ON: PsciReturnCode = -4;
pub const PSCI_ERROR_ON_PENDING: PsciReturnCode = -5;
pub const PSCI_ERROR_INTERNAL_FAILURE: PsciReturnCode = -6;
pub const PSCI_ERROR_NOT_PRESENT: PsciReturnCode = -7;
pub const PSCI_ERROR_DISABLED: PsciReturnCode = -8;
pub const PSCI_ERROR_INVALID_ADDRESS: PsciReturnCode = -9;

pub type EntryPoint = i64;
pub type ContextId = u64;
pub type CpuState = u64;
pub type MemoryState = u64;
pub type SecurityState = u64;
pub type TimerState = u64;
pub type InterruptState = u64;
pub type PowerState = u64;
pub type CoherencyState = u64;
pub type CacheState = u64;

pub struct S {
    pub cpu_default_suspend_entry_point: EntryPoint,
    pub cpu_default_suspend_context: ContextId,
    pub cpu_state: CpuState,
    pub memory_state: MemoryState,
    pub security_state: SecurityState,
    pub timer_state: TimerState,
    pub interrupt_state: InterruptState,
    pub power_state: PowerState,
    pub coherency_state: CoherencyState,
    pub cache_state: CacheState,
}

} // verus!

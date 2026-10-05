use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub type CoreState = u64;

pub type PsciCoordinationMode = u64;

pub struct S {
    pub cmd_input_mode: u64,
    pub psci_coordination_mode: PsciCoordinationMode,
    pub num_cores: u32,
    pub calling_core: u32,
}

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub const RSI_ERROR_NOT_SUPPORTED: RsiCommandReturnCode = 3;

pub const PSCI_COORDINATION_PLATFORM: PsciCoordinationMode = 0;
pub const PSCI_COORDINATION_OS: PsciCoordinationMode = 1;

pub const OFF: CoreState = 0;
pub const NOT_BOOTED: CoreState = 1;
pub const SUSPENDED: CoreState = 2;
pub const RUNNING: CoreState = 3;

pub open spec fn CoreStateAt(s: S, c: u32) -> CoreState;

} // verus!

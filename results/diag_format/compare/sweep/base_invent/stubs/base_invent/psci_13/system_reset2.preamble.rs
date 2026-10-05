use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const PSCI_NOT_SUPPORTED: PsciReturnCode = -1;
pub const PSCI_ERROR_INVALID_PARAMETERS: PsciReturnCode = -2;

pub type PsciFeatureStatus = i32;

pub const PSCI_FEATURE_SUPPORTED: PsciFeatureStatus = 1;
pub const PSCI_FEATURE_NOT_SUPPORTED: PsciFeatureStatus = -1;

pub struct PsciFeatureSet {
    pub system_reset2: PsciFeatureStatus,
}

pub struct CpuState {
    pub value: u64,
}

pub struct MemoryState {
    pub value: u64,
}

pub struct InterruptState {
    pub value: u64,
}

pub struct MmusState {
    pub value: u64,
}

pub struct SmmusState {
    pub value: u64,
}

pub struct PsciState {
    pub cmd_input_reset_type: u32,
    pub cpu_state: CpuState,
    pub memory_state: MemoryState,
    pub interrupt_state: InterruptState,
    pub mmus_state: MmusState,
    pub smmus_state: SmmusState,
}

pub open spec fn PsciFeatures(s: PsciState) -> PsciFeatureSet;

} // verus!

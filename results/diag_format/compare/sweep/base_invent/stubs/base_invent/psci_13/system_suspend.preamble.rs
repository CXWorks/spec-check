use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const PSCI_NOT_SUPPORTED: PsciReturnCode = -1;
pub const PSCI_INVALID_PARAMETERS: PsciReturnCode = -2;
pub const PSCI_DENIED: PsciReturnCode = -3;
pub const PSCI_ALREADY_ON: PsciReturnCode = -4;
pub const PSCI_ON_PENDING: PsciReturnCode = -5;
pub const PSCI_INTERNAL_FAILURE: PsciReturnCode = -6;
pub const PSCI_NOT_PRESENT: PsciReturnCode = -7;
pub const PSCI_DISABLED: PsciReturnCode = -8;
pub const PSCI_INVALID_ADDRESS: PsciReturnCode = -9;

pub struct PsciState {
    pub cpu_power_states: Map<u64, u32>,
    pub system_suspended: bool,
    pub entry_point: u64,
    pub context_id: u64,
}

} // verus!

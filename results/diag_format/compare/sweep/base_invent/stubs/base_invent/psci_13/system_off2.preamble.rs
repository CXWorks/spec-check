use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;

pub const PSCI_RETURN_SUCCESS: PsciReturnCode = 0;
pub const PSCI_RETURN_NOT_SUPPORTED: PsciReturnCode = -1;
pub const PSCI_RETURN_INVALID_PARAMETERS: PsciReturnCode = -2;
pub const PSCI_RETURN_DENIED: PsciReturnCode = -3;
pub const PSCI_RETURN_ALREADY_ON: PsciReturnCode = -4;
pub const PSCI_RETURN_ON_PENDING: PsciReturnCode = -5;
pub const PSCI_RETURN_INTERNAL_FAILURE: PsciReturnCode = -6;
pub const PSCI_RETURN_NOT_PRESENT: PsciReturnCode = -7;
pub const PSCI_RETURN_DISABLED: PsciReturnCode = -8;
pub const PSCI_RETURN_INVALID_ADDRESS: PsciReturnCode = -9;

pub struct PsciState {
    pub system_off: bool,
}

} // verus!

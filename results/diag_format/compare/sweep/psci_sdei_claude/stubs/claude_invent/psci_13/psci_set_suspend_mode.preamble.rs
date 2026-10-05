use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const PSCI_NOT_SUPPORTED: PsciReturnCode = -1;
pub const PSCI_INVALID_PARAMETERS: PsciReturnCode = -2;
pub const PSCI_DENIED: PsciReturnCode = -3;

pub struct S {
    pub dummy: int,
}

pub open spec fn PsciSetSuspendModeSupported(s: S) -> bool;

pub open spec fn PsciSuspendMode(s: S) -> UInt32;

pub open spec fn AllCoresRunningOffOrDefaultSuspended(s: S) -> bool;

pub open spec fn AnyCoreCalledCpuSuspendSinceLastModeChangeOrBoot(s: S) -> bool;

pub open spec fn AllCoresOtherThanCallerOff(s: S) -> bool;

} // verus!

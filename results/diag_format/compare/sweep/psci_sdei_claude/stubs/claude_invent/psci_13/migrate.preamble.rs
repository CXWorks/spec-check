use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const PSCI_NOT_SUPPORTED: PsciReturnCode = -1;
pub const PSCI_INVALID_PARAMETERS: PsciReturnCode = -2;
pub const PSCI_DENIED: PsciReturnCode = -3;
pub const PSCI_NOT_PRESENT: PsciReturnCode = -7;
pub const PSCI_INTERNAL_FAILURE: PsciReturnCode = -6;

pub struct S {
    pub placeholder: u64,
}

pub open spec fn PsciMigrateSupported(s: S) -> bool;
pub open spec fn TrustedOsMigrationRequired(s: S) -> bool;
pub open spec fn IsValidMpidr(s: S, mpidr: UInt64) -> bool;
pub open spec fn CallerRegisterWidthMatchesFunctionId(s: S) -> bool;
pub open spec fn TrustedOsMigrateCapable(s: S) -> bool;
pub open spec fn CurrentCpu(s: S) -> UInt64;
pub open spec fn TrustedOsResidentCpu(s: S) -> UInt64;

} // verus!

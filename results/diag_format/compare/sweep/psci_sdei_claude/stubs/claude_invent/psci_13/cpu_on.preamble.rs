use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const PSCI_INVALID_PARAMETERS: PsciReturnCode = -2;
pub const PSCI_DENIED: PsciReturnCode = -3;
pub const PSCI_ALREADY_ON: PsciReturnCode = -4;
pub const PSCI_ON_PENDING: PsciReturnCode = -5;
pub const PSCI_INTERNAL_FAILURE: PsciReturnCode = -6;
pub const PSCI_INVALID_ADDRESS: PsciReturnCode = -9;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsValidMpidr(s: S, mpidr: UInt64) -> bool;

pub open spec fn IsEntryPointKnownInvalid(s: S, addr: UInt64) -> bool;

pub open spec fn CoreIsOn(s: S, mpidr: UInt64) -> bool;

pub open spec fn CoreIsOnPending(s: S, mpidr: UInt64) -> bool;

pub open spec fn CoreIsDeniedByFirmwarePolicy(s: S, mpidr: UInt64) -> bool;

pub open spec fn CoreEntryPoint(s: S, mpidr: UInt64) -> UInt64;

pub open spec fn CoreContextId(s: S, mpidr: UInt64) -> UInt64;

} // verus!

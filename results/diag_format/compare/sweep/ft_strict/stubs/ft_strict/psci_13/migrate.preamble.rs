use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;
pub const NOT_PRESENT: PsciReturnCode = -7;
pub const INTERNAL_FAILURE: PsciReturnCode = -6;

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual(result: PsciReturnCode, code: PsciReturnCode) -> bool;

pub open spec fn MigrateIsImplemented(s: S) -> bool;

pub open spec fn MigrationRequired(s: S) -> bool;

pub open spec fn IsValidMpidr(s: S, mpidr: UInt64) -> bool;

pub open spec fn CallerRegisterWidth(s: S) -> u64;

pub open spec fn FidRegisterWidth(s: S, idx: u64) -> u64;

pub open spec fn TrustedOsIsUp(s: S) -> bool;

pub open spec fn TrustedOsIsMigrateCapable(s: S) -> bool;

pub open spec fn CurrentCore(s: S) -> UInt64;

pub open spec fn TrustedOsResidentCore(s: S) -> UInt64;

pub open spec fn MigrateImplementationDefinedFailure(s: S, target_cpu: UInt64) -> bool;

} // verus!

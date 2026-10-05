use vstd::prelude::*;

verus! {

pub type UInt = u64;
pub type Mpidr = u64;
pub type PsciReturnCode = i64;
pub type RegisterWidth = u64;

pub struct S {
    pub dummy: u64,
}

pub const RSI_SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;
pub const NOT_PRESENT: PsciReturnCode = -7;
pub const INTERNAL_FAILURE: PsciReturnCode = -6;

pub open spec fn MigrateIsImplemented(s: S) -> bool;
pub open spec fn MigrationIsRequired(s: S) -> bool;
pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;
pub open spec fn IsValidMpidr(s: S, mpidr: Mpidr) -> bool;
pub open spec fn CallerRegisterWidth(s: S) -> RegisterWidth;
pub open spec fn FidRegisterWidth(s: S, fid: UInt) -> RegisterWidth;
pub open spec fn TrustedOsIsUp(s: S) -> bool;
pub open spec fn TrustedOsIsMigrateCapable(s: S) -> bool;
pub open spec fn MigrateImplementationDefinedFailure(s: S, target_cpu: Mpidr) -> bool;
pub open spec fn CurrentCpu(s: S) -> Mpidr;
pub open spec fn TrustedOsResidentCore(s: S) -> Mpidr;

} // verus!

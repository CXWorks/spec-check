use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Mpidr = u64;
pub type PsciReturnCode = i32;
pub type RegisterWidth = u32;

pub struct S {
    pub dummy: u64,
}

pub spec const NOT_SUPPORTED: PsciReturnCode = -1i32;
pub spec const INVALID_PARAMETERS: PsciReturnCode = -2i32;
pub spec const DENIED: PsciReturnCode = -3i32;
pub spec const NOT_PRESENT: PsciReturnCode = -7i32;
pub spec const INTERNAL_FAILURE: PsciReturnCode = -6i32;

pub uninterp spec fn MigrateIsImplemented(s: S) -> bool;
pub uninterp spec fn MigrationIsRequired(s: S) -> bool;
pub uninterp spec fn ResultEqual(r: PsciReturnCode, c: PsciReturnCode) -> bool;
pub uninterp spec fn IsValidMpidr(s: S, m: Mpidr) -> bool;
pub uninterp spec fn CallerRegisterWidth(s: S) -> RegisterWidth;
pub uninterp spec fn FidRegisterWidth(fid: UInt64) -> RegisterWidth;
pub uninterp spec fn TrustedOsIsUp(s: S) -> bool;
pub uninterp spec fn TrustedOsIsMigrateCapable(s: S) -> bool;
pub uninterp spec fn MigrateImplementationDefinedFailure(s: S, m: Mpidr) -> bool;
pub uninterp spec fn CurrentCpu(s: S) -> Mpidr;
pub uninterp spec fn TrustedOsResidentCore(s: S) -> Mpidr;

} // verus!

use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;
pub type MpidrType = u64;
pub type FunctionId = u32;
pub type RegisterWidth = u32;

pub struct S {
    pub target_cpu: MpidrType,
}

pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;
pub const INTERNAL_FAILURE: PsciReturnCode = -6;
pub const NOT_PRESENT: PsciReturnCode = -7;

#[allow(non_upper_case_globals)]
pub const target_cpu: MpidrType = 1;
#[allow(non_upper_case_globals)]
pub const fid: FunctionId = 2;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;
pub open spec fn MigrateIsImplemented(s: S) -> bool;
pub open spec fn MigrationIsRequired(s: S) -> bool;
pub open spec fn IsValidMpidr(s: S, cpu: MpidrType) -> bool;
pub open spec fn CallerRegisterWidth(s: S) -> RegisterWidth;
pub open spec fn FidRegisterWidth(f: FunctionId) -> RegisterWidth;
pub open spec fn TrustedOsIsUp(s: S) -> bool;
pub open spec fn TrustedOsIsMigrateCapable(s: S) -> bool;
pub open spec fn MigrateImplementationDefinedFailure(s: S, cpu: MpidrType) -> bool;
pub open spec fn CurrentCpu(s: S) -> MpidrType;
pub open spec fn TrustedOsResidentCore(s: S) -> MpidrType;

} // verus!

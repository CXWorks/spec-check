use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;
pub type UInt64 = u64;
pub type CoreId = u64;
pub type RegisterWidth = u32;

pub struct S {
    pub trusted_os_resident_core: CoreId,
    pub current_core: CoreId,
    pub trusted_os_up: bool,
    pub trusted_os_migrate_capable: bool,
}

pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;
pub const INTERNAL_FAILURE: PsciReturnCode = -6;
pub const NOT_PRESENT: PsciReturnCode = -7;

pub spec const target_cpu: UInt64 = 0;
pub spec const fid: UInt64 = 0xC4000005;

pub open spec fn MigrateIsImplemented(s: S) -> bool;
pub open spec fn MigrationRequired(s: S) -> bool;
pub open spec fn IsValidMpidr(s: S, mpidr: UInt64) -> bool;
pub open spec fn CallerRegisterWidth(s: S) -> RegisterWidth;
pub open spec fn FidRegisterWidth(f: UInt64) -> RegisterWidth;
pub open spec fn TrustedOsIsUp(s: S) -> bool;
pub open spec fn TrustedOsIsMigrateCapable(s: S) -> bool;
pub open spec fn CurrentCore(s: S) -> CoreId;
pub open spec fn TrustedOsResidentCore(s: S) -> CoreId;
pub open spec fn MigrateImplementationDefinedFailure(s: S, mpidr: UInt64) -> bool;
pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

} // verus!

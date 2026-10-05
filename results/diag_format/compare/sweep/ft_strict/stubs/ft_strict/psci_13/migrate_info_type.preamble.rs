use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const DENIED: Int64 = -3;

pub open spec fn MigrateInfoTypeImplemented() -> bool;
pub open spec fn TrustedOsMigrateInfoType() -> Int64;
pub open spec fn TrustedOsIsUniprocessor() -> bool;
pub open spec fn TrustedOsIsMigrateCapable() -> bool;
pub open spec fn TrustedOsIsMpCapable() -> bool;
pub open spec fn TrustedOsPresent() -> bool;
pub open spec fn MigrateToValidTargetReturns(code: Int64) -> bool;
pub open spec fn CpuOffOnResidentCoreReturns(code: Int64) -> bool;
pub open spec fn MigrateHasNoEffect() -> bool;
pub open spec fn CpuOffOnResidentCoreDoesNotReturn() -> bool;
pub open spec fn TrustedOsResidentCore() -> UInt64;
pub open spec fn MpidrOf(core: UInt64) -> Int64;
pub open spec fn IsTargetCpuFormat(v: Int64) -> bool;
pub open spec fn IsUndefinedValue(v: Int64) -> bool;

} // verus!

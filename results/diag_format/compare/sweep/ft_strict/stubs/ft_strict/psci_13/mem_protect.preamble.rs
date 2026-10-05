use vstd::prelude::*;

verus! {

pub type UInt64 = u64;
pub type Int64 = i64;

pub struct S {
    pub mem_protect_enabled: bool,
    pub mem_protect_implemented: bool,
}

pub const NOT_SUPPORTED: Int64 = -1;

pub open spec fn MemProtectImplemented(s: S) -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn Old(b: bool) -> bool;

pub open spec fn MemProtectEnabled(s: S) -> bool;

pub open spec fn MemProtectCheckRangeImplemented(s: S) -> bool;

pub open spec fn AllCallerAccessibleVolatileMemoryOverwrittenOnNextNonWarmResetBoot(s: S) -> bool;

pub open spec fn CallerMemoryPreservedOnSystemWarmReset(s: S) -> bool;

pub open spec fn MemProtectDisabledBeforeOsBootLoaderHandover(s: S) -> bool;

} // verus!

use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub struct S {
    pub mem_protect_enabled: bool,
}

pub const NOT_SUPPORTED: Int64 = -2;

pub const enable: u64 = 1;

pub open spec fn MemProtectImplemented() -> bool;

pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;

pub open spec fn Old(b: bool) -> bool;

pub open spec fn MemProtectEnabled() -> bool;

pub open spec fn MemProtectCheckRangeImplemented() -> bool;

pub open spec fn AllCallerAccessibleVolatileMemoryOverwrittenOnNextNonWarmResetBoot() -> bool;

pub open spec fn CallerMemoryPreservedOnSystemWarmReset() -> bool;

pub open spec fn MemProtectDisabledBeforeOsBootLoaderHandover() -> bool;

} // verus!

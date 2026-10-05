use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct S {
    pub os_initiated_mode: bool,
    pub stat_functions_implemented: bool,
}

pub spec const NOT_SUPPORTED: int = -1;

pub spec const target_cpu: UInt64 = 1;

pub spec const power_state: UInt64 = 2;

pub spec const fid: UInt64 = 3;

pub open spec fn StatFunctionsImplemented(s: S) -> bool;

pub open spec fn ResultEqual(result: UInt64, value: int) -> bool;

pub open spec fn IsNodePresent(s: S, cpu: UInt64) -> bool;

pub open spec fn NodeSupportsState(s: S, cpu: UInt64, state: UInt64) -> bool;

pub open spec fn IsStatCountFid(f: UInt64) -> bool;

pub open spec fn IsStatResidencyFid(f: UInt64) -> bool;

pub open spec fn StatCount(s: S, cpu: UInt64, local_state: UInt64) -> int;

pub open spec fn StatResidencyMicroseconds(s: S, cpu: UInt64, local_state: UInt64) -> int;

pub open spec fn HighestLevelLocalState(s: S, state: UInt64) -> UInt64;

pub open spec fn ResultModulus(f: UInt64) -> int;

pub open spec fn IsOsInitiatedMode(s: S) -> bool;

pub open spec fn IgnoreLastManField(state: UInt64) -> UInt64;

pub open spec fn IsRunState(s: S, local_state: UInt64) -> bool;

pub open spec fn StatsIncludeAllEntryMethods(s: S, cpu: UInt64, local_state: UInt64) -> bool;

} // verus!

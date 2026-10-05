use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;

pub open spec fn StatFunctionsImplemented(s: S) -> bool;

pub open spec fn ResultEqual(result: UInt64, expected: u64) -> bool;

pub open spec fn IsNodePresent(s: S, target_cpu: UInt64) -> bool;

pub open spec fn NodeSupportsState(s: S, target_cpu: UInt64, power_state: UInt64) -> bool;

pub open spec fn IsStatCountFid(s: S, fid: UInt64) -> bool;

pub open spec fn IsStatResidencyFid(s: S, fid: UInt64) -> bool;

pub open spec fn StatCount(s: S, target_cpu: UInt64, local_state: u64) -> u64;

pub open spec fn StatResidencyMicroseconds(s: S, target_cpu: UInt64, local_state: u64) -> u64;

pub open spec fn HighestLevelLocalState(s: S, power_state: UInt64) -> u64;

pub open spec fn ResultModulus(s: S, fid: UInt64) -> u64;

pub open spec fn IsOsInitiatedMode(s: S) -> bool;

pub open spec fn IgnoreLastManField(s: S, power_state: UInt64) -> UInt64;

pub open spec fn IsRunState(s: S, local_state: u64) -> bool;

pub open spec fn StatsIncludeAllEntryMethods(s: S, target_cpu: UInt64, local_state: u64) -> bool;

} // verus!

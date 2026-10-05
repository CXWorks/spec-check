use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i64;

pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DISABLED: PsciReturnCode = -8;
pub const ON: PsciReturnCode = 0;
pub const OFF: PsciReturnCode = 1;
pub const ON_PENDING: PsciReturnCode = 2;

pub struct PsciVersionT {
    pub major: int,
    pub minor: int,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn IsAffinityInstancePresent(s: S, lowest_affinity_level: UInt64, target_affinity: UInt64) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn PsciVersion(s: S) -> PsciVersionT;

pub open spec fn SupportsAffinityLevelAboveZero(s: S) -> bool;

pub open spec fn IsAffinityInstanceDisabled(s: S, lowest_affinity_level: UInt64, target_affinity: UInt64) -> bool;

pub open spec fn AffinityInstance(s: S, lowest_affinity_level: UInt64, target_affinity: UInt64) -> Set<int>;

pub open spec fn CoreEnabledByCpuOn(s: S, core: int) -> bool;

pub open spec fn IsColdBootPrimaryCore(s: S, core: int) -> bool;

pub open spec fn CoreCalledCpuOff(s: S, core: int) -> bool;

pub open spec fn CpuOffProcessed(s: S, core: int) -> bool;

pub open spec fn CoreState(s: S, core: int) -> PsciReturnCode;

} // verus!

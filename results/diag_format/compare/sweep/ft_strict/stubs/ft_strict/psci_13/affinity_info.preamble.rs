use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i64;

pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DISABLED: PsciReturnCode = -8;
pub const ON: PsciReturnCode = 0;
pub const OFF: PsciReturnCode = 1;
pub const ON_PENDING: PsciReturnCode = 2;

pub struct S {
    pub supports_affinity_level_above_zero: bool,
}

pub struct Core {
    pub mpidr: u64,
}

pub open spec fn AffinityInstanceIsPresent(s: S, target_affinity: UInt64, lowest_affinity_level: UInt64) -> bool;

pub open spec fn ResultEqual(result: PsciReturnCode, code: PsciReturnCode) -> bool;

pub open spec fn SupportsAffinityLevelAboveZero(s: S) -> bool;

pub open spec fn AffinityInstanceIsDisabled(s: S, target_affinity: UInt64, lowest_affinity_level: UInt64) -> bool;

pub open spec fn InAffinityInstance(c: Core, target_affinity: UInt64, lowest_affinity_level: UInt64) -> bool;

pub open spec fn CoreEnabledByCpuOn(c: Core) -> bool;

pub open spec fn IsColdBootPrimaryCore(c: Core) -> bool;

pub open spec fn CoreHasCalledCpuOff(c: Core) -> bool;

pub open spec fn CpuOffProcessed(c: Core) -> bool;

pub open spec fn CoreIsOnPending(c: Core) -> bool;

} // verus!

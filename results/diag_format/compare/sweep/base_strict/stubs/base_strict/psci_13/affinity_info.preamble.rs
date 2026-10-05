use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub type Core = u64;

pub type MPIDR = u64;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: PsciReturnCode = 0;
pub const ON: PsciReturnCode = 0;
pub const OFF: PsciReturnCode = 1;
pub const ON_PENDING: PsciReturnCode = 2;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DISABLED: PsciReturnCode = -8;

pub const target_affinity: MPIDR = 0;
pub const lowest_affinity_level: u32 = 0;

pub open spec fn ResultEqual(result: PsciReturnCode, code: PsciReturnCode) -> bool;

pub open spec fn AffinityInstanceIsPresent(s: S, ta: MPIDR, lal: u32) -> bool;

pub open spec fn SupportsAffinityLevelAboveZero() -> bool;

pub open spec fn AffinityInstanceIsDisabled(s: S, ta: MPIDR, lal: u32) -> bool;

pub open spec fn InAffinityInstance(c: Core, ta: MPIDR, lal: u32) -> bool;

pub open spec fn CoreEnabledByCpuOn(c: Core) -> bool;

pub open spec fn IsColdBootPrimaryCore(c: Core) -> bool;

pub open spec fn CoreHasCalledCpuOff(c: Core) -> bool;

pub open spec fn CpuOffProcessed(c: Core) -> bool;

pub open spec fn CoreIsOnPending(c: Core) -> bool;

} // verus!

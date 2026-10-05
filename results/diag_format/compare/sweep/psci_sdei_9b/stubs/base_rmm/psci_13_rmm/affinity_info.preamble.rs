use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub type CoreId = u64;

pub type PsciCoreState = u64;

pub struct S {
    pub cores: Set<CoreId>,
}

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub spec const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub spec const ON_PENDING: PsciCoreState = 2;
pub spec const OFF: PsciCoreState = 3;

pub spec const lowest_affinity_level: UInt64 = 4;
pub spec const target_affinity: UInt64 = 5;

pub open spec fn ResultEqual(result: RsiCommandReturnCode, expected: RsiCommandReturnCode) -> bool;

pub open spec fn IsAffinityInstancePresent(s: S, level: UInt64, target: UInt64) -> bool;

pub open spec fn IsAffinityInstanceDisabled(s: S, level: UInt64, target: UInt64) -> bool;

pub open spec fn PsciVersion() -> int;

pub open spec fn SupportsAffinityLevelAboveZero() -> bool;

pub open spec fn AffinityInstance(s: S, level: UInt64, target: UInt64) -> Set<CoreId>;

pub open spec fn CoreEnabledByCpuOn(core: CoreId) -> bool;

pub open spec fn IsColdBootPrimaryCore(core: CoreId) -> bool;

pub open spec fn CoreCalledCpuOff(core: CoreId) -> bool;

pub open spec fn CpuOffProcessed(core: CoreId) -> bool;

pub open spec fn CoreState(core: CoreId) -> PsciCoreState;

} // verus!

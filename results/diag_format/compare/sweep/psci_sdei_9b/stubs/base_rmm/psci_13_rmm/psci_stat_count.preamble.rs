use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type CpuId = u64;

pub type PowerState = u32;

pub struct S {
    pub target_cpu_field: CpuId,
    pub power_state_field: PowerState,
    pub stat_counter: u64,
}

pub const NOT_SUPPORTED: i64 = -1;

pub open spec fn StatFunctionsImplemented() -> bool;

pub open spec fn ResultEqual(result: UInt64, code: i64) -> bool;

pub open spec fn target_cpu(s: S) -> CpuId;

pub open spec fn power_state(s: S) -> PowerState;

pub open spec fn IsPresentNode(cpu: CpuId) -> bool;

pub open spec fn NodeSupportsState(cpu: CpuId, state: PowerState) -> bool;

pub open spec fn IsOsInitiatedMode() -> bool;

pub open spec fn LastManLevelField(state: PowerState) -> u32;

} // verus!

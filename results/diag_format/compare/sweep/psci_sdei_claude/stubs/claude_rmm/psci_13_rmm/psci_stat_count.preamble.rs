use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Mpidr = u64;
pub type PowerState = u32;
pub type LocalState = u32;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: i64 = -1;
pub const PSCI_STAT_RESIDENCY: u64 = 0xC4000010;
pub const PSCI_STAT_COUNT: u64 = 0xC4000011;

pub open spec fn StatFunctionsImplemented() -> bool;
pub open spec fn ResultEqual(result: UInt64, code: i64) -> bool;
pub open spec fn IsPresentNode(target_cpu: Mpidr) -> bool;
pub open spec fn NodeSupportsState(target_cpu: Mpidr, power_state: PowerState) -> bool;
pub open spec fn HighestLevelLocalState(power_state: PowerState) -> LocalState;
pub open spec fn StatCount(target_cpu: Mpidr, state: LocalState) -> UInt64;
pub open spec fn StatResidencyMicroseconds(target_cpu: Mpidr, state: LocalState) -> UInt64;

} // verus!

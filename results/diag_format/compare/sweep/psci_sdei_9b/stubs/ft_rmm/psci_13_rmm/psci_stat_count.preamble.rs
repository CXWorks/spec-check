use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Mpidr = u64;
pub type PowerState = u32;
pub type LocalState = u32;

pub struct S {
    pub stat_functions_implemented: bool,
    pub os_initiated_mode: bool,
}

pub open spec fn StatFunctionsImplemented(s: S) -> bool;

pub open spec fn IsPresentNode(s: S, target_cpu: Mpidr) -> bool;

pub open spec fn NodeSupportsState(s: S, target_cpu: Mpidr, power_state: PowerState) -> bool;

pub open spec fn HighestLevelLocalState(s: S, power_state: PowerState) -> LocalState;

pub open spec fn StatCount(s: S, target_cpu: Mpidr, local_state: LocalState) -> UInt64;

pub open spec fn IsOsInitiatedMode(s: S) -> bool;

pub open spec fn LastManLevelField(s: S, power_state: PowerState) -> u32;

} // verus!

use vstd::prelude::*;
verus! {

pub type UInt = int;

pub type Mpidr = u64;

pub type PowerState = u32;

pub type PowerLevel = nat;

pub type LocalState = nat;

pub type NodeId = nat;

pub struct S {
    pub os_initiated_mode: bool,
    pub stat_functions_implemented: bool,
}

pub const NOT_SUPPORTED: int = -1;

pub open spec fn StatFunctionsImplemented(s: S) -> bool;

pub open spec fn ResultEqual(result: UInt, value: int) -> bool;

pub open spec fn IsPresentNode(s: S, target_cpu: Mpidr) -> bool;

pub open spec fn NodeSupportsState(s: S, node: NodeId, local_state: LocalState) -> bool;

pub open spec fn StatNode(s: S, target_cpu: Mpidr, power_state: PowerState) -> NodeId;

pub open spec fn StatLocalState(s: S, power_state: PowerState) -> LocalState;

pub open spec fn NodeAt(s: S, target_cpu: Mpidr, level: PowerLevel) -> NodeId;

pub open spec fn HighestPowerLevel(s: S, power_state: PowerState) -> PowerLevel;

pub open spec fn IsOsInitiatedMode(s: S) -> bool;

pub open spec fn LastManLevelField(s: S, power_state: PowerState) -> PowerLevel;

pub open spec fn IsStatCount(s: S, result: UInt) -> bool;

pub open spec fn IsStatResidency(s: S, result: UInt) -> bool;

pub open spec fn StateUseCount(s: S, node: NodeId, local_state: LocalState) -> int;

pub open spec fn StateResidencyUs(s: S, node: NodeId, local_state: LocalState) -> int;

pub open spec fn ResultBits(s: S, result: UInt) -> nat;

} // verus!

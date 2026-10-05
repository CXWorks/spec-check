use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub struct Node {
    pub id: int,
}

pub struct LocalState {
    pub id: int,
}

pub spec const NOT_SUPPORTED: int = -1;

pub open spec fn ResultEqual(result: UInt64, v: int) -> bool;

pub open spec fn StatFunctionsImplemented(s: S) -> bool;

pub open spec fn NodePresent(s: S, n: Node) -> bool;

pub open spec fn StatNode(target_cpu: UInt64, power_state: UInt64) -> Node;

pub open spec fn NodeSupportsLocalState(s: S, n: Node, ls: LocalState) -> bool;

pub open spec fn StatLocalState(power_state: UInt64) -> LocalState;

pub open spec fn IsStatCount(s: S, result: UInt64) -> bool;

pub open spec fn IsStatResidency(s: S, result: UInt64) -> bool;

pub open spec fn StatEntryCount(s: S, n: Node, ls: LocalState) -> int;

pub open spec fn StatResidencyMicroseconds(s: S, n: Node, ls: LocalState) -> int;

pub open spec fn ResultModulus(result: UInt64) -> int;

pub open spec fn IsOsInitiatedMode(s: S) -> bool;

pub open spec fn DisregardLastInLevelField(power_state: UInt64) -> UInt64;

pub open spec fn IsLocalLowPowerState(s: S, ls: LocalState) -> bool;

pub open spec fn StatsCountAllEntryMethods(s: S, n: Node, ls: LocalState) -> bool;

pub open spec fn StatsZeroedAtColdBoot(s: S) -> bool;

pub open spec fn StatsZeroedBySystemResetAndShutdown(s: S) -> bool;

} // verus!

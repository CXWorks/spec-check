use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type NodeId = int;

pub type LocalState = int;

pub struct S {
    pub dummy: int,
}

pub spec const NOT_SUPPORTED: int = -1int;

pub spec const target_cpu: UInt64 = 1u64;

pub spec const power_state: UInt64 = 2u64;

pub spec const fid: UInt64 = 3u64;

pub open spec fn StatFunctionsImplemented() -> bool;

pub open spec fn ResultEqual(result: UInt64, expected: int) -> bool;

pub open spec fn NodePresent(node: NodeId) -> bool;

pub open spec fn StatNode(s: S, cpu: UInt64, pstate: UInt64) -> NodeId;

pub open spec fn NodeSupportsLocalState(node: NodeId, ls: LocalState) -> bool;

pub open spec fn StatLocalState(pstate: UInt64) -> LocalState;

pub open spec fn IsStatCount(f: UInt64) -> bool;

pub open spec fn IsStatResidency(f: UInt64) -> bool;

pub open spec fn StatEntryCount(node: NodeId, ls: LocalState) -> int;

pub open spec fn StatResidencyMicroseconds(node: NodeId, ls: LocalState) -> int;

pub open spec fn ResultModulus(f: UInt64) -> int;

pub open spec fn IsOsInitiatedMode() -> bool;

pub open spec fn DisregardLastInLevelField(pstate: UInt64) -> UInt64;

pub open spec fn IsLocalLowPowerState(ls: LocalState) -> bool;

pub open spec fn StatsCountAllEntryMethods(node: NodeId, ls: LocalState) -> bool;

pub open spec fn StatsZeroedAtColdBoot() -> bool;

pub open spec fn StatsZeroedBySystemResetAndShutdown() -> bool;

} // verus!

use vstd::prelude::*;

verus! {

pub type ReturnCode = u64;
pub type PowerStateT = u64;
pub type CacheState = u64;
pub type CoherencyState = u64;

pub struct S {
    pub dummy: u64,
}

pub struct Node {
    pub id: u64,
    pub level: nat,
}

pub spec const RSI_SUCCESS: ReturnCode = 0;
pub spec const DENIED: ReturnCode = 1;
pub spec const result: ReturnCode = 2;

pub spec const POWERED_DOWN: PowerStateT = 0;

pub spec const node: Node = Node { id: 0, level: 0 };

pub open spec fn CallingCore() -> Node;

pub open spec fn IsTrustedOsResident(s: S, n: Node) -> bool;

pub open spec fn ResultEqual(a: ReturnCode, b: ReturnCode) -> bool;

pub open spec fn ReturnsToCaller(s: S) -> bool;

pub open spec fn PowerState(s: S, n: Node) -> PowerStateT;

pub open spec fn CachesClean(s: S, n: Node) -> bool;

pub open spec fn IsCoherent(s: S, n: Node) -> bool;

pub open spec fn AllCoresCalledCpuOff(s: S, n: Node) -> bool;

pub open spec fn RemainingCoresSuspendedAtOrAbove(s: S, level: nat) -> bool;

pub open spec fn Caches(s: S, n: Node) -> CacheState;

pub open spec fn Coherency(s: S, n: Node) -> CoherencyState;

} // verus!

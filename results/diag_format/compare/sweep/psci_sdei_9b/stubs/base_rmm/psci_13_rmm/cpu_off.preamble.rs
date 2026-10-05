use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;

pub spec const SUCCESS: PsciReturnCode = 0;
pub spec const NOT_SUPPORTED: PsciReturnCode = -1;
pub spec const INVALID_PARAMETERS: PsciReturnCode = -2;
pub spec const DENIED: PsciReturnCode = -3;

pub type PowerStateValue = u8;

pub spec const POWERED_ON: PowerStateValue = 0;
pub spec const RETENTION: PowerStateValue = 1;
pub spec const POWERED_DOWN: PowerStateValue = 2;

pub type CacheState = u8;

pub spec const CachesDirty: CacheState = 0;
pub spec const CachesClean: CacheState = 1;

pub type Level = nat;

pub struct Node {
    pub id: nat,
    pub level: Level,
}

pub struct S {
    pub id: nat,
}

pub open spec fn IsTrustedOsResident(s: S, node: Node) -> bool;

pub open spec fn CallingCore() -> Node;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn ReturnsToCaller() -> bool;

pub open spec fn PowerState(s: S, node: Node) -> PowerStateValue;

pub open spec fn Caches(s: S, node: Node) -> CacheState;

pub open spec fn IsCoherent(s: S, node: Node) -> bool;

pub open spec fn AllCoresCalledCpuOff(s: S, node: Node) -> bool;

pub open spec fn RemainingCoresSuspendedAtOrAbove(s: S, level: Level) -> bool;

} // verus!

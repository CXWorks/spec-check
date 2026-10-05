use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub type Core = u64;

pub type TopologyNode = u64;

pub type PowerLevelType = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const DENIED: PsciReturnCode = (-3int) as i32;

pub spec const calling_cpu: Core = 0;

pub open spec fn IsTrustedOsResidentCore(s: S, c: Core) -> bool;

pub open spec fn ResultEqual(r1: PsciReturnCode, r2: PsciReturnCode) -> bool;

pub open spec fn CorePoweredDown(s: S, c: Core) -> bool;

pub open spec fn CallReturns(s: S, c: Core) -> bool;

pub open spec fn NodePoweredDown(n: TopologyNode) -> bool;

pub open spec fn CachesCleaned(n: TopologyNode) -> bool;

pub open spec fn CoreInCoherency(s: S, c: Core) -> bool;

pub open spec fn IsCluster(n: TopologyNode) -> bool;

pub open spec fn ClusterInCoherency(n: TopologyNode) -> bool;

pub open spec fn ContainsCore(n: TopologyNode, c: Core) -> bool;

pub open spec fn CalledCpuOff(c: Core) -> bool;

pub open spec fn PowerLevel(n: TopologyNode) -> PowerLevelType;

pub open spec fn RequestedPowerdownAtOrAbove(c: Core, level: PowerLevelType) -> bool;

} // verus!

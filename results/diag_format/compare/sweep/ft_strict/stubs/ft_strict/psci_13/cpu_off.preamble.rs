use vstd::prelude::*;

verus! {

pub type Cpu = u64;

pub type Core = Cpu;

pub type PsciReturnCode = i32;

pub type TopologyNode = u64;

pub type PowerLevel = u64;

pub type PowerStateValue = u64;

pub type CacheStateValue = u64;

pub type CoherencyStateValue = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const PSCI_SUCCESS: PsciReturnCode = 0;

pub spec const DENIED: PsciReturnCode = -3;

pub open spec fn IsTrustedOsResidentCore(s: S, cpu: Cpu) -> bool;

pub open spec fn CorePoweredDown(s: S, cpu: Cpu) -> bool;

pub open spec fn CallReturns(s: S, cpu: Cpu) -> bool;

pub open spec fn NodePoweredDown(s: S, n: TopologyNode) -> bool;

pub open spec fn CachesCleaned(s: S, n: TopologyNode) -> bool;

pub open spec fn CoreInCoherency(s: S, cpu: Cpu) -> bool;

pub open spec fn IsCluster(s: S, n: TopologyNode) -> bool;

pub open spec fn ClusterInCoherency(s: S, n: TopologyNode) -> bool;

pub open spec fn ContainsCore(s: S, n: TopologyNode, c: Core) -> bool;

pub open spec fn CalledCpuOff(s: S, c: Core) -> bool;

pub open spec fn PowerState(s: S, n: TopologyNode) -> PowerLevel;

pub open spec fn RequestedPowerdownAtOrAbove(s: S, c: Core, level: PowerLevel) -> bool;

pub open spec fn CorePowerState(s: S, cpu: Cpu) -> PowerStateValue;

pub open spec fn CacheState(s: S, cpu: Cpu) -> CacheStateValue;

pub open spec fn CoherencyState(s: S, cpu: Cpu) -> CoherencyStateValue;

pub open spec fn AncestorNodes(s: S, cpu: Cpu) -> Seq<TopologyNode>;

pub open spec fn NodePowerState(s: S, nodes: Seq<TopologyNode>) -> Seq<PowerStateValue>;

} // verus!

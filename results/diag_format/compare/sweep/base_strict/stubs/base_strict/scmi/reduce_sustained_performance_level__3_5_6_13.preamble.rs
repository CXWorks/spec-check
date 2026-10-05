use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type AgentId = u32;

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const NOT_FOUND: int32 = -4;
pub const DENIED: int32 = -3;
pub const OUT_OF_RANGE: int32 = -6;

pub const REDUCE_SUSTAINED_PERFORMANCE_LEVEL: uint32 = 0x10;

pub const agent: AgentId = 1;

pub struct PerfDomainState {
    pub provisioned_sustained_level: uint32,
}

pub struct PerfDomainAttrs {
    pub sustained_perf_level: uint32,
}

pub struct S {
    pub dummy: uint32,
}

impl S {
    pub open spec fn PerfDomain(self, domain_id: uint32) -> PerfDomainState;
}

pub open spec fn IsCommandSupported(cmd: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn IsValidPerfDomain(domain_id: uint32) -> bool;

pub open spec fn AgentPermittedToReduceSustainedLevel(a: AgentId, domain_id: uint32) -> bool;

pub open spec fn LevelOrIndex(domain_id: uint32, level: uint32) -> uint32;

pub open spec fn IsInAllowedSustainedRange(domain_id: uint32, level: uint32) -> bool;

pub open spec fn ReducedSustainedLevelScheduled(domain_id: uint32, level: uint32) -> bool;

pub open spec fn PerfDomainAttributes(domain_id: uint32) -> PerfDomainAttrs;

pub open spec fn PlatformSustainedPerfLevel(domain_id: uint32) -> uint32;

} // verus!

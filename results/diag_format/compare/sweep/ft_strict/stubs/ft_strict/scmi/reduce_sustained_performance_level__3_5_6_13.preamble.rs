use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;
pub type AgentId = u32;
pub type CommandId = u32;

pub struct S {
    pub dummy: u32,
}

pub struct PerfDomainAttrs {
    pub sustained_perf_level: uint32,
}

pub spec const REDUCE_SUSTAINED_PERFORMANCE_LEVEL: CommandId = 13;

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_SUPPORTED: int32 = (-1) as int32;
pub spec const DENIED: int32 = (-3) as int32;
pub spec const NOT_FOUND: int32 = (-4) as int32;
pub spec const OUT_OF_RANGE: int32 = (-5) as int32;

#[allow(non_upper_case_globals)]
pub spec const agent: AgentId = 7;

#[allow(non_upper_case_globals)]
pub spec const result: int32 = 100;

pub uninterp spec fn IsCommandSupported(s: S, cmd: CommandId) -> bool;

pub uninterp spec fn ResultEqual(status: int32, code: int32) -> bool;

pub uninterp spec fn IsValidPerfDomain(s: S, domain_id: uint32) -> bool;

pub uninterp spec fn AgentPermittedToReduceSustainedLevel(s: S, agent_id: AgentId, domain_id: uint32) -> bool;

pub uninterp spec fn LevelOrIndex(s: S, domain_id: uint32, level: uint32) -> uint32;

pub uninterp spec fn IsInAllowedSustainedRange(s: S, domain_id: uint32, level: uint32) -> bool;

pub uninterp spec fn ReducedSustainedLevelScheduled(s: S, domain_id: uint32, level: uint32) -> bool;

pub uninterp spec fn PerfDomainAttributes(s: S, domain_id: uint32) -> PerfDomainAttrs;

pub uninterp spec fn PlatformSustainedPerfLevel(s: S, domain_id: uint32) -> uint32;

} // verus!

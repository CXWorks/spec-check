use vstd::prelude::*;

verus! {

pub type uint32 = u32;
pub type int32 = i32;
pub type AgentId = u32;
pub type MessageId = u32;

pub struct PerfDomainAttrs {
    pub attributes: Seq<u32>,
    pub sustained_perf_level: u32,
}

pub struct S {
    pub dummy: int,
}

pub const REDUCE_SUSTAINED_PERFORMANCE_LEVEL: MessageId = 0x10;

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const DENIED: int32 = -3;
pub const NOT_FOUND: int32 = -4;
pub const OUT_OF_RANGE: int32 = -5;

pub const agent: AgentId = 7;
pub const result: int32 = -99;

pub open spec fn IsCommandSupported(s: S, msg: MessageId) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn IsValidPerfDomain(s: S, domain_id: uint32) -> bool;

pub open spec fn PerfDomainAttributes(s: S, domain_id: uint32) -> PerfDomainAttrs;

pub open spec fn AgentMayReduceSustainedLevel(s: S, agent_id: AgentId, domain_id: uint32) -> bool;

pub open spec fn IsInAllowedRange(s: S, domain_id: uint32, level: uint32) -> bool;

pub open spec fn PlatformHasAcceptedAndScheduled(s: S, domain_id: uint32, level: uint32) -> bool;

} // verus!

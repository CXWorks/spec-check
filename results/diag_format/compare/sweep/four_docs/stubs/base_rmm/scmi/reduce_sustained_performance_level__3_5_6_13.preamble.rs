use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub struct PerfDomainAttrs {
    pub attributes: Seq<u8>,
    pub sustained_perf_level: uint32,
}

pub type AgentId = uint32;

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_SUPPORTED: int32 = -1i32;
pub spec const NOT_FOUND: int32 = -3i32;
pub spec const DENIED: int32 = -4i32;
pub spec const OUT_OF_RANGE: int32 = -2i32;

pub spec const REDUCE_SUSTAINED_PERFORMANCE_LEVEL: uint32 = 13;

pub spec const domain_id: uint32 = 101;
pub spec const sustained_level: uint32 = 202;
pub spec const agent: AgentId = 303;

pub uninterp spec fn IsCommandSupported(cmd: uint32) -> bool;
pub uninterp spec fn ResultEqual(result: int32, code: int32) -> bool;
pub uninterp spec fn IsValidPerfDomain(domain: uint32) -> bool;
pub uninterp spec fn PerfDomainAttributes(domain: uint32) -> PerfDomainAttrs;
pub uninterp spec fn AgentMayReduceSustainedLevel(a: AgentId, domain: uint32) -> bool;
pub uninterp spec fn IsInAllowedRange(domain: uint32, level: uint32) -> bool;
pub uninterp spec fn PlatformHasAcceptedAndScheduled(domain: uint32, level: uint32) -> bool;
pub uninterp spec fn old<T>(x: T) -> T;

} // verus!

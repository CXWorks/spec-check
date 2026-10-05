use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct PowercapDomainInfo {
    pub mai: UInt32,
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub uninterp spec fn unknown_result() -> Int32;

pub uninterp spec fn unknown_calling_agent() -> AgentId;

pub spec const result: Int32 = unknown_result();

pub spec const calling_agent: AgentId = unknown_calling_agent();

pub uninterp spec fn PowercapDomainExists(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn IsMaiSetSupported(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn AreValidMaiSetFlags(s: S, flags: UInt32) -> bool;

pub uninterp spec fn IsSupportedMai(s: S, domain_id: UInt32, mai: UInt32) -> bool;

pub uninterp spec fn AgentMaySetMai(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub uninterp spec fn PowercapDomain(s: S, domain_id: UInt32) -> PowercapDomainInfo;

} // verus!

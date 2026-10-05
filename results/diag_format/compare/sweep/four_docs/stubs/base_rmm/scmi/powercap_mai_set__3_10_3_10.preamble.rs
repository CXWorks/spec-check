use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct PowercapDomainInfo {
    pub mai: UInt32,
}

pub struct S {
    pub num_domains: UInt32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1) as Int32;
pub spec const INVALID_PARAMETERS: Int32 = (-2) as Int32;
pub spec const DENIED: Int32 = (-3) as Int32;
pub spec const NOT_FOUND: Int32 = (-4) as Int32;

pub spec const domain_id: UInt32 = 0;
pub spec const flags: UInt32 = 1;
pub spec const mai: UInt32 = 2;
pub spec const calling_agent: UInt32 = 3;

pub open spec fn PowercapDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsMaiSetSupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn AreValidMaiSetFlags(s: S, flags: UInt32) -> bool;

pub open spec fn IsSupportedMai(s: S, domain_id: UInt32, mai: UInt32) -> bool;

pub open spec fn AgentMaySetMai(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn PowercapDomain(s: S, domain_id: UInt32) -> PowercapDomainInfo;

} // verus!

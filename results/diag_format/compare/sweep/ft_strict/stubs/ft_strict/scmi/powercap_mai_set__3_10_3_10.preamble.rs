use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2) as i32;
pub spec const DENIED: Int32 = (-3) as i32;
pub spec const NOT_FOUND: Int32 = (-4) as i32;
pub spec const result: Int32 = (-100) as i32;

pub spec const calling_agent: AgentId = 0;

pub uninterp spec fn PowercapDomainExists(s: S, domain_id: UInt32) -> bool;
pub uninterp spec fn IsMaiSetSupported(s: S, domain_id: UInt32) -> bool;
pub uninterp spec fn IsSupportedMai(s: S, domain_id: UInt32, mai: UInt32) -> bool;
pub uninterp spec fn AreValidMaiSetFlags(s: S, flags: UInt32) -> bool;
pub uninterp spec fn AgentMaySetMai(s: S, agent: AgentId, domain_id: UInt32) -> bool;
pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub uninterp spec fn PowercapDomainMai(s: S, domain_id: UInt32) -> UInt32;

} // verus!

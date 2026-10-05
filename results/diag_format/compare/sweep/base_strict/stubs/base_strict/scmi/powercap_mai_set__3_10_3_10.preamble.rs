use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: UInt32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;
pub spec const DENIED: Int32 = (-3int) as i32;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;

pub spec const domain_id: UInt32 = 1;
pub spec const mai: UInt32 = 2;
pub spec const flags: UInt32 = 3;
pub spec const calling_agent: UInt32 = 4;

pub uninterp spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub uninterp spec fn PowercapDomainExists(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn IsMaiSetSupported(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn IsSupportedMai(s: S, domain_id: UInt32, mai: UInt32) -> bool;

pub uninterp spec fn AreValidMaiSetFlags(s: S, flags: UInt32) -> bool;

pub uninterp spec fn AgentMaySetMai(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub uninterp spec fn PowercapDomainMai(s: S, domain_id: UInt32) -> UInt32;

} // verus!

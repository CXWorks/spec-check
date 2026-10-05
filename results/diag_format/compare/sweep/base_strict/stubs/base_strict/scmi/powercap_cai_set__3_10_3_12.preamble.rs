use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type AgentId = u32;

pub struct S {
    pub placeholder: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;
pub spec const DENIED: Int32 = (-3int) as i32;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;

pub spec const domain_id: UInt32 = 0;
pub spec const cpli: UInt32 = 1;
pub spec const cai: UInt32 = 2;
pub spec const flags: UInt32 = 3;

pub uninterp spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub uninterp spec fn IsValidPowercapDomainId(s: S, domain_id: UInt32) -> bool;
pub uninterp spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;
pub uninterp spec fn IsCaiSetSupported(s: S, domain_id: UInt32) -> bool;
pub uninterp spec fn IsSupportedCai(s: S, domain_id: UInt32, cai: UInt32) -> bool;
pub uninterp spec fn CallingAgent() -> AgentId;
pub uninterp spec fn AgentMaySetCai(s: S, agent_id: AgentId, domain_id: UInt32) -> bool;
pub uninterp spec fn PowercapCai(s: S, domain_id: UInt32, cpli: UInt32) -> UInt32;

} // verus!

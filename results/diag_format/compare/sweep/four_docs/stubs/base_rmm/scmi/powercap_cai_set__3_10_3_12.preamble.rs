use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct PowercapDomainInfo {
    pub cai: UInt32,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;
pub spec const DENIED: Int32 = (-3int) as i32;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;

pub spec const domain_id: UInt32 = 1;
pub spec const cpli: UInt32 = 2;
pub spec const cai: UInt32 = 3;
pub spec const flags: UInt32 = 4;
pub spec const calling_agent: UInt32 = 5;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;
pub open spec fn IsCaiSetSupported(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsSupportedCai(s: S, domain_id: UInt32, cai: UInt32) -> bool;
pub open spec fn AreValidCaiSetFlags(s: S, flags: UInt32) -> bool;
pub open spec fn AgentMaySetCai(s: S, agent: UInt32, domain_id: UInt32) -> bool;
pub open spec fn PowercapDomain(s: S, domain_id: UInt32) -> PowercapDomainInfo;

} // verus!

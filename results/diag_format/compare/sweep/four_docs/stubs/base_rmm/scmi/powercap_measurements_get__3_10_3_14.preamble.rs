use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type AgentId = u32;
pub type DomainId = u32;

pub struct PowercapDomainInfo {
    pub average_power_over_latest_mai: uint32,
    pub mai: uint32,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_SUPPORTED: int32 = (-1int) as i32;
pub spec const NOT_FOUND: int32 = (-3int) as i32;
pub spec const DENIED: int32 = (-15int) as i32;

pub spec const domain_id: DomainId = 0;
pub spec const caller: AgentId = 1;

pub uninterp spec fn IsValidPowercapDomain(s: S, domain_id: DomainId) -> bool;
pub uninterp spec fn IsPowercapMeasurementsGetSupported(s: S, domain_id: DomainId) -> bool;
pub uninterp spec fn AgentMayGetPowercapMeasurements(s: S, caller: AgentId, domain_id: DomainId) -> bool;
pub uninterp spec fn ResultEqual(result: int32, code: int32) -> bool;
pub uninterp spec fn PowercapDomain(s: S, domain_id: DomainId) -> PowercapDomainInfo;

} // verus!

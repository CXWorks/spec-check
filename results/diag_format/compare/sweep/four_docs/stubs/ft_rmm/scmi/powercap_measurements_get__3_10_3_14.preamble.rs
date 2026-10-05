use vstd::prelude::*;

verus! {

pub type uint32 = u32;
pub type int32 = i32;
pub type AgentId = u32;

pub struct PowercapDomainInfo {
    pub average_power_over_latest_mai: uint32,
    pub mai: uint32,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_SUPPORTED: int32 = (-1int) as int32;
pub spec const NOT_FOUND: int32 = (-4int) as int32;
pub spec const DENIED: int32 = (-3int) as int32;

#[allow(non_upper_case_globals)]
pub spec const caller: AgentId = 0;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: uint32) -> bool;

pub open spec fn IsPowercapMeasurementsGetSupported(s: S, domain_id: uint32) -> bool;

pub open spec fn AgentMayGetPowercapMeasurements(s: S, agent: AgentId, domain_id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn PowercapDomain(s: S, domain_id: uint32) -> PowercapDomainInfo;

} // verus!

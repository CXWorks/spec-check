use vstd::prelude::*;

verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct VoltageDomainInfo {
    pub level: int32,
}

pub struct S {
    pub num_domains: uint32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const NOT_FOUND: int32 = -4;
pub const DENIED: int32 = -3;

pub const caller: uint32 = 0;

pub open spec fn IsValidVoltageDomain(s: S, domain_id: uint32) -> bool;

pub open spec fn IsRequestSupported(s: S, domain_id: uint32) -> bool;

pub open spec fn AgentMayGetVoltageLevel(s: S, agent_id: uint32, domain_id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn VoltageDomain(s: S, domain_id: uint32) -> VoltageDomainInfo;

} // verus!

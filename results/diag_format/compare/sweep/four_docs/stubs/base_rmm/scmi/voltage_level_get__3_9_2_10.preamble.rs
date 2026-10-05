use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type UInt32 = u32;

pub struct VoltageDomainInfo {
    pub level: int32,
}

pub struct S {
    pub domain_id_field: UInt32,
    pub caller_field: UInt32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const NOT_FOUND: int32 = -4;
pub const DENIED: int32 = -3;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn caller(s: S) -> UInt32;

pub open spec fn IsValidVoltageDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsRequestSupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn AgentMayGetVoltageLevel(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: int32, expected: int32) -> bool;

pub open spec fn VoltageDomain(s: S, domain_id: UInt32) -> VoltageDomainInfo;

} // verus!

use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct VoltageDomainInfo {
    pub mode: UInt32,
}

pub struct S {
    pub dummy: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

#[allow(non_upper_case_globals)]
pub const result: Int32 = 99;

#[allow(non_upper_case_globals)]
pub const caller: UInt32 = 0;

pub open spec fn VoltageDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn VoltageDomainSupportsConfig(s: S, domain_id: UInt32, config: [UInt32; 2]) -> bool;

pub open spec fn IsRequestSupported(s: S) -> bool;

pub open spec fn AgentMaySetVoltageConfig(s: S, agent: UInt32, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn VoltageDomain(s: S, domain_id: UInt32) -> VoltageDomainInfo;

} // verus!

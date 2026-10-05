use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const NOT_FOUND: int32 = -4;
pub const DENIED: int32 = -3;

#[allow(non_upper_case_globals)]
pub const calling_agent: uint32 = 0;

pub open spec fn IsValidVoltageDomain(s: S, domain_id: uint32) -> bool;

pub open spec fn IsRequestSupported(s: S, protocol_id: uint32, message_id: uint32) -> bool;

pub open spec fn AgentMayGetVoltageConfig(s: S, agent: uint32, domain_id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn Bits(x: uint32, hi: nat, lo: nat) -> uint32;

pub open spec fn VoltageDomainMode(s: S, domain_id: uint32) -> uint32;

} // verus!

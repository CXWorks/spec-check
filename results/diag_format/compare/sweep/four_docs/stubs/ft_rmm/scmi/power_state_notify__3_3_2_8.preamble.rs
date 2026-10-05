use vstd::prelude::*;
verus! {

pub type UInt32 = Seq<u32>;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;

#[allow(non_upper_case_globals)]
pub const result: Int32 = 7;

#[allow(non_upper_case_globals)]
pub const calling_agent: AgentId = 0;

pub open spec fn IsValidPowerDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PowerStateNotifyEnabled(s: S, agent: AgentId, domain_id: UInt32) -> bool;

} // verus!

use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub calling_agent_field: u32,
    pub domain_id_field: u32,
    pub notify_enable_field: Seq<u8>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn calling_agent(s: S) -> UInt32;

pub open spec fn notify_enable(s: S) -> Seq<u8>;

pub open spec fn IsValidPowerDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: Seq<u8>) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn PowerStateNotifyEnabled(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

} // verus!

use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type UInt32 = u32;
pub type UInt8 = u8;

pub struct S {
    pub domain_id_field: UInt32,
    pub notify_enable_field: Seq<UInt8>,
    pub calling_agent_field: UInt32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = 1;
pub const INVALID_PARAMETERS: int32 = 2;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn notify_enable(s: S) -> Seq<UInt8>;

pub open spec fn calling_agent(s: S) -> UInt32;

pub open spec fn IsValidResetDomain(domain_id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(notify_enable: Seq<UInt8>) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn ResetNotifyEnabled(domain_id: UInt32, agent: UInt32) -> bool;

} // verus!

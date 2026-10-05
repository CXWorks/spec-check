use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;
pub const INVALID_PARAMETERS: int32 = -2;

pub const domain_id: uint32 = 10;
pub const calling_agent: uint32 = 20;
pub const notify_enable: [uint32; 1] = [1];

pub uninterp spec fn IsValidPowercapDomain(s: S, d: uint32) -> bool;
pub uninterp spec fn IsValidNotifyEnable(s: S, ne: [uint32; 1]) -> bool;
pub uninterp spec fn ResultEqual(a: int32, b: int32) -> bool;
pub uninterp spec fn PowercapCapNotifyEnabled(s: S, agent_id: uint32) -> uint32;

} // verus!

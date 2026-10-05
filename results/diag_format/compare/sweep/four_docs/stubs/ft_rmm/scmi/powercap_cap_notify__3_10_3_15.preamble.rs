use vstd::prelude::*;

verus! {

pub type uint32 = Seq<u32>;
pub type int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const INVALID_PARAMETERS: int32 = (-2int) as i32;
pub spec const NOT_FOUND: int32 = (-4int) as i32;

pub spec const result: int32 = 1;
pub spec const calling_agent: u32 = 0;

pub uninterp spec fn IsValidPowercapDomain(s: S, domain_id: uint32) -> bool;

pub uninterp spec fn IsValidNotifyEnable(s: S, notify_enable: uint32) -> bool;

pub uninterp spec fn ResultEqual(a: int32, b: int32) -> bool;

pub uninterp spec fn PowercapCapNotifyEnabled(s: S, domain_id: uint32, agent: u32) -> u32;

} // verus!

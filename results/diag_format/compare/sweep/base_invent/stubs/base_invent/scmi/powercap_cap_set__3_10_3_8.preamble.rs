use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct PowercapDomain {
    pub cpli: uint32,
    pub supports_cpc: bool,
    pub flags: uint32,
    pub power_cap: uint32,
}

pub struct S {
    pub powercap_domains: Seq<PowercapDomain>,
    pub domain_id: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;

pub open spec fn ResultEqual(result: int32, expected: int32) -> bool;

} // verus!

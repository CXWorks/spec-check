use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = 1;
pub const NOT_SUPPORTED: i32 = 2;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapMaiGetSupported(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapDomainMai(s: S, domain_id: u32) -> u32;

} // verus!

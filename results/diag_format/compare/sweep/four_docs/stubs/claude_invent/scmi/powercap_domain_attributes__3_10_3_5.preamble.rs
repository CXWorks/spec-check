use vstd::prelude::*;
verus! {

pub struct S {
    pub domains: Set<u32>,
}

pub const SUCCESS: i32 = 0i32;
pub const NOT_FOUND: i32 = -4i32;

pub open spec fn PowercapDomainExists(s: S, domain_id: u32) -> bool;

} // verus!

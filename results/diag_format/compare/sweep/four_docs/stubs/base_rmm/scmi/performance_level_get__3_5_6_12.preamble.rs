use vstd::prelude::*;
verus! {

pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int = 0;
pub spec const NOT_FOUND: int = 1;

pub uninterp spec fn IsValidPerformanceDomain(domain_id: uint32) -> bool;

pub uninterp spec fn ResultEqual(status: int, code: int) -> bool;

pub uninterp spec fn CurrentPerformanceLevel(domain_id: uint32) -> uint32;

} // verus!

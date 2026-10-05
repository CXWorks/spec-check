use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -6;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: uint32) -> bool;

pub open spec fn ResultEqual(status: int32, code: int32) -> bool;

pub open spec fn CurrentPerformanceLevel(s: S, domain_id: uint32) -> uint32;

} // verus!

use vstd::prelude::*;

verus! {

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -1;

pub struct S {
    pub dummy: u32,
}

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: u32) -> bool;

pub open spec fn PerformanceLimitMax(s: S, domain_id: u32) -> u32;

pub open spec fn PerformanceLimitMin(s: S, domain_id: u32) -> u32;

} // verus!

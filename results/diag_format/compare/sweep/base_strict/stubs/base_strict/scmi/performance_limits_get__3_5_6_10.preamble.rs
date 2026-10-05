use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub struct S {
    pub domain_id: u32,
    pub limit_max: u32,
    pub limit_min: u32,
}

pub const SUCCESS: Int32 = 0;

pub const NOT_FOUND: Int32 = -1;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn IsValidPerformanceDomain(domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn CurrentPerformanceLimitMax(domain_id: UInt32) -> UInt32;

pub open spec fn CurrentPerformanceLimitMin(domain_id: UInt32) -> UInt32;

} // verus!

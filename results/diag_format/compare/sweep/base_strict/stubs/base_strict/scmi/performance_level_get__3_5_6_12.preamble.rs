use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub performance_domain_count: UInt32,
    pub current_performance_level: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;

#[allow(non_upper_case_globals)]
pub const domain_id: UInt32 = 0;

pub open spec fn IsValidPerformanceDomain(s: S, did: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn CurrentPerformanceLevelOrIndex(did: UInt32) -> UInt32;

} // verus!

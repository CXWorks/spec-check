use vstd::prelude::*;

verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct S {
    pub dummy: int,
}

pub struct PerformanceLimits {
    pub max: uint32,
    pub min: uint32,
}

pub struct PerformanceDomainInfo {
    pub limits: PerformanceLimits,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_FOUND: int32 = (-4int) as int32;
pub spec const result: int32 = 7;

pub open spec fn IsValidPerformanceDomain(s: S, domain_id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn PerformanceDomain(s: S, domain_id: uint32) -> PerformanceDomainInfo;

} // verus!

use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type ScmiStatus = i32;

pub const SUCCESS: ScmiStatus = 0;
pub const NOT_FOUND: ScmiStatus = -4;
pub const OUT_OF_RANGE: ScmiStatus = -6;
pub const DENIED: ScmiStatus = -3;

pub struct S {
    pub dummy: int,
}

pub open spec fn PerfDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerfHighestLevel(s: S, domain_id: UInt32) -> int;

pub open spec fn PerfLowestLevel(s: S, domain_id: UInt32) -> int;

pub open spec fn PerfLimitsSetPermitted(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerfLimitMax(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PerfLimitMin(s: S, domain_id: UInt32) -> UInt32;

} // verus!

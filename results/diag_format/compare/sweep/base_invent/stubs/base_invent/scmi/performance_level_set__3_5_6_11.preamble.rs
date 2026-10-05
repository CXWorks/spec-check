use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub num_domains: nat,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;
pub const OUT_OF_RANGE: int32 = -6;
pub const DENIED: int32 = -3;

pub const domain_id: uint32 = 0;
pub const performance_level: uint32 = 1;

pub uninterp spec fn DomainExists(s: S, d_id: uint32) -> bool;

pub uninterp spec fn PerformanceLevelInRange(s: S, d_id: uint32, p_level: uint32) -> bool;

pub uninterp spec fn AgentPermittedToChangePerformanceLevel(s: S, d_id: uint32, p_level: uint32) -> bool;

} // verus!

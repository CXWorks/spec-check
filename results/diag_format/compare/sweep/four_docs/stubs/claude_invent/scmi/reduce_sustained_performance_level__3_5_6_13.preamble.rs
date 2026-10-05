use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -3;
pub const NOT_FOUND: i32 = -4;
pub const OUT_OF_RANGE: i32 = -5;

pub open spec fn ReduceSustainedPerfLevelCommandSupported(s: S) -> bool;

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsQosOnlyPerfDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn AgentPermittedToReduceSustainedPerfLevel(s: S, domain_id: UInt32) -> bool;

pub open spec fn SustainedLevelInAllowedRange(s: S, domain_id: UInt32, sustained_level: UInt32) -> bool;

pub open spec fn SustainedLevelExceedsPlatformSustainedLevel(s: S, domain_id: UInt32, sustained_level: UInt32) -> bool;

pub open spec fn ReducedSustainedLevelRequestScheduled(s: S, domain_id: UInt32, sustained_level: UInt32) -> bool;

pub open spec fn PlatformSustainedPerfLevel(s: S, domain_id: UInt32) -> UInt32;

} // verus!

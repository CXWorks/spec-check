use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const OUT_OF_RANGE: i32 = -3;
pub const DENIED: i32 = -4;
pub const NOT_FOUND: i32 = -5;

pub open spec fn VoltageDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn VoltageLevelIndexInRange(s: S, domain_id: UInt32, level_index: UInt32) -> bool;

pub open spec fn AgentAllowedToGetVoltageLevels(s: S, domain_id: UInt32) -> bool;

pub open spec fn VoltageDescribeLevelsRequestSupported(s: S, domain_id: UInt32) -> bool;

pub open spec fn VoltageDomainLowestLevel(s: S, domain_id: UInt32) -> int;

pub open spec fn VoltageDomainHighestLevel(s: S, domain_id: UInt32) -> int;

pub open spec fn VoltageDomainStepSize(s: S, domain_id: UInt32) -> int;

pub open spec fn VoltageDomainLevelCount(s: S, domain_id: UInt32) -> int;

pub open spec fn VoltageDomainLevelAt(s: S, domain_id: UInt32, index: int) -> int;

} // verus!

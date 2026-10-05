use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -3;

#[allow(non_upper_case_globals)]
pub const result: Int32 = 0;

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn PerfLevelsStartAfterSkipped(s: S, domain_id: UInt32, skip_index: UInt32, perf_levels: Seq<[UInt32; 5]>) -> bool;
pub open spec fn NumRemainingPerfLevels(s: S, domain_id: UInt32, skip_index: UInt32, n: UInt32) -> UInt32;
pub open spec fn PerfLevelValue(perf_levels: Seq<[UInt32; 5]>, i: int) -> UInt32;
pub open spec fn PerfLevelAttributes(perf_levels: Seq<[UInt32; 5]>, i: int) -> UInt32;
pub open spec fn PerfLevelPowerCost(perf_levels: Seq<[UInt32; 5]>, i: int) -> UInt32;
pub open spec fn PerfLevelIndicativeFreq(perf_levels: Seq<[UInt32; 5]>, i: int) -> UInt32;
pub open spec fn PerfLevelIndex(perf_levels: Seq<[UInt32; 5]>, i: int) -> UInt32;
pub open spec fn WorstCaseTransitionLatencyUs(s: S, domain_id: UInt32, level: UInt32) -> UInt32;
pub open spec fn IsLinearPowerScale(s: S, domain_id: UInt32, cost: UInt32) -> bool;
pub open spec fn DomainClockFrequencyKhz(s: S, domain_id: UInt32, level: UInt32) -> UInt32;
pub open spec fn LevelIndexingModeEnabled(s: S, domain_id: UInt32) -> bool;
pub open spec fn LevelIndexOf(s: S, domain_id: UInt32, level: UInt32) -> UInt32;

} // verus!

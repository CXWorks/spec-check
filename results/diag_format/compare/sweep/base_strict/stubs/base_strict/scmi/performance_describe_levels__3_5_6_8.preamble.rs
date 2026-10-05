use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type Array<T, const N: usize> = [T; N];

pub struct S {
    pub dummy: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = -6i32;

pub spec const domain_id: UInt32 = 0;
pub spec const skip_index: UInt32 = 1;

pub open spec fn IsValidPerfDomain(s: S, d: UInt32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn PerfLevelsStartAfterSkipped(d: UInt32, skip: UInt32, levels: Array<UInt32, 5>) -> bool;
pub open spec fn NumRemainingPerfLevels(d: UInt32, skip: UInt32, n: UInt32) -> UInt32;
pub open spec fn PerfLevelValue<I>(levels: Array<UInt32, 5>, i: I) -> UInt32;
pub open spec fn PerfLevelAttributes<I>(levels: Array<UInt32, 5>, i: I) -> UInt32;
pub open spec fn WorstCaseTransitionLatencyUs(d: UInt32, level: UInt32) -> UInt32;
pub open spec fn PerfLevelPowerCost<I>(levels: Array<UInt32, 5>, i: I) -> UInt32;
pub open spec fn IsLinearPowerScale(d: UInt32, cost: UInt32) -> bool;
pub open spec fn PerfLevelIndicativeFreq<I>(levels: Array<UInt32, 5>, i: I) -> UInt32;
pub open spec fn DomainClockFrequencyKhz(d: UInt32, level: UInt32) -> UInt32;
pub open spec fn LevelIndexingModeEnabled(d: UInt32) -> bool;
pub open spec fn PerfLevelIndex<I>(levels: Array<UInt32, 5>, i: I) -> UInt32;
pub open spec fn LevelIndexOf(d: UInt32, level: UInt32) -> UInt32;

} // verus!

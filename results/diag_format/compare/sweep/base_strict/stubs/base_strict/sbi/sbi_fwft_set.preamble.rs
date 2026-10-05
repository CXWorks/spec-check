use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type UInt32 = u32;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub feature_values: Map<u32, u64>,
    pub feature_values_at_entry: Map<u32, u64>,
    pub feature_locked: Map<u32, bool>,
}

pub const feature: u32 = 0;
pub const value: u64 = 1;
pub const flags: u64 = 2;

pub open spec fn FeatureValue(s: S, f: u32) -> u64;
pub open spec fn FeatureValueAtEntry(s: S, f: u32) -> u64;
pub open spec fn IsSuccessfulReturn(r: sbiret) -> bool;
pub open spec fn Bits(x: u64, hi: int, lo: int) -> int;
pub open spec fn FeatureIsLocked(s: S, f: u32) -> bool;
pub open spec fn FeatureHasLocalScope(s: S, f: u32) -> bool;
pub open spec fn FeatureHasGlobalScope(s: S, f: u32) -> bool;
pub open spec fn FeatureValueImmutableUntilHartReset(s: S, f: u32) -> bool;
pub open spec fn FeatureValueImmutableUntilSystemReset(s: S, f: u32) -> bool;

} // verus!

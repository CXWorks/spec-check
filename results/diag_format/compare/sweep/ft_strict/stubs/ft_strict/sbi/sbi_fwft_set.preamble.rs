use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type unsigned_long = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn FeatureValue(s: S, feature: UInt32) -> u64;

pub open spec fn FeatureValueAtEntry(s: S, feature: UInt32) -> u64;

pub open spec fn IsSuccessfulReturn(s: S, ret: sbiret) -> bool;

pub open spec fn Bits(x: u64, hi: int, lo: int) -> int;

pub open spec fn FeatureIsLocked(s: S, feature: UInt32) -> bool;

pub open spec fn FeatureHasLocalScope(s: S, feature: UInt32) -> bool;

pub open spec fn FeatureHasGlobalScope(s: S, feature: UInt32) -> bool;

pub open spec fn FeatureValueImmutableUntilHartReset(s: S, feature: UInt32) -> bool;

pub open spec fn FeatureValueImmutableUntilSystemReset(s: S, feature: UInt32) -> bool;

} // verus!

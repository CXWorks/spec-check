use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub dummy: u64,
}

pub spec const feature: UInt32 = 7;

pub spec const value: UInt64 = 3;

pub spec const flags: UInt64 = 2;

pub spec const LOCK: UInt64 = 1;

pub open spec fn IsSuccessfulReturn(r: sbiret) -> bool;

pub open spec fn FeatureValue(f: UInt32) -> UInt64;

pub open spec fn FlagIsSet(fl: UInt64, mask: UInt64) -> bool;

pub open spec fn FeatureIsLocked(f: UInt32) -> bool;

} // verus!

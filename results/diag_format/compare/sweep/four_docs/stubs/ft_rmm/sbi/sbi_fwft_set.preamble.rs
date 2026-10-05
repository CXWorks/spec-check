use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type unsigned_long = u64;

pub struct sbiret {
    pub error: i64,
    pub result: u64,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn IsSuccessfulReturn(result: sbiret) -> bool;

pub open spec fn FeatureValue(s: S, feature: uint32) -> unsigned_long;

pub open spec fn FlagIsSet(flags: unsigned_long, bit: int) -> bool;

pub open spec fn FeatureIsLocked(s: S, feature: uint32) -> bool;

} // verus!

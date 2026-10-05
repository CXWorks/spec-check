use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct SbiRet {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: i64 = 0i64;

pub open spec fn FwftFeatureLocked(s: S, feature: UInt32) -> bool;

pub open spec fn FwftFeatureValue(s: S, feature: UInt32) -> UInt64;

pub open spec fn FwftFeatureSupported(s: S, feature: UInt32) -> bool;

pub open spec fn FwftFeatureValueSupported(feature: UInt32, value: UInt64) -> bool;

} // verus!

use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt = u64;

pub struct S {
    pub dummy: int,
}

pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsReservedFeature(s: S, feature: UInt32) -> bool;

pub open spec fn IsValidFeature(s: S, feature: UInt32) -> bool;

pub open spec fn PlatformSupportsFeature(s: S, feature: UInt32) -> bool;

pub open spec fn IsPlatformSpecificFeature(s: S, feature: UInt32) -> bool;

pub open spec fn IsImplementedFeature(s: S, feature: UInt32) -> bool;

pub open spec fn FeatureGetFailed(s: S, feature: UInt32) -> bool;

pub open spec fn FeatureConfigValue(s: S, feature: UInt32) -> UInt;

} // verus!

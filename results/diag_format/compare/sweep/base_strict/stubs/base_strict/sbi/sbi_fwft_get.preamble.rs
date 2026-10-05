use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type SbiErrorCode = i64;

pub struct S {
    pub features_config: Map<UInt, UInt>,
    pub reserved: Set<UInt>,
    pub implemented: Set<UInt>,
}

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;
pub const SBI_ERR_DENIED: SbiErrorCode = -4;

pub const feature: UInt = 0;

pub open spec fn IsReservedFeature(s: S, f: UInt) -> bool;

pub open spec fn IsValidFeature(s: S, f: UInt) -> bool;

pub open spec fn PlatformSupportsFeature(s: S, f: UInt) -> bool;

pub open spec fn IsPlatformSpecificFeature(s: S, f: UInt) -> bool;

pub open spec fn IsImplementedFeature(s: S, f: UInt) -> bool;

pub open spec fn FeatureGetFailed(s: S, f: UInt) -> bool;

pub open spec fn FeatureConfigValue(s: S, f: UInt) -> UInt;

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

} // verus!

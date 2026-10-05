use vstd::prelude::*;
verus! {

pub type SbiCommandReturnCode = i64;

pub struct S {
    pub dummy: u64,
}

pub spec const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub spec const SBI_ERR_FAILED: SbiCommandReturnCode = (-1) as i64;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiCommandReturnCode = (-2) as i64;
pub spec const SBI_ERR_DENIED: SbiCommandReturnCode = (-4) as i64;

pub open spec fn ResultEqual(a: SbiCommandReturnCode, b: SbiCommandReturnCode) -> bool;

pub open spec fn IsReservedFeature(feature: u32) -> bool;

pub open spec fn IsValidFeature(feature: u32) -> bool;

pub open spec fn PlatformSupportsFeature(feature: u32) -> bool;

pub open spec fn IsPlatformSpecificFeature(feature: u32) -> bool;

pub open spec fn IsImplementedFeature(feature: u32) -> bool;

pub open spec fn FeatureGetFailed(feature: u32) -> bool;

pub open spec fn FeatureConfigValue(feature: u32) -> u32;

} // verus!

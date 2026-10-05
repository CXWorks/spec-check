use vstd::prelude::*;
verus! {

pub type uint32_t = u32;
pub type uint64_t = u64;
pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiCommandReturnCode = -2;
pub const SBI_ERR_DENIED: SbiCommandReturnCode = -4;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsReservedFeature(s: S, feature: uint32_t) -> bool;
pub open spec fn IsValidFeature(s: S, feature: uint32_t) -> bool;
pub open spec fn PlatformSupportsFeature(s: S, feature: uint32_t) -> bool;
pub open spec fn IsPlatformSpecificFeature(s: S, feature: uint32_t) -> bool;
pub open spec fn IsImplementedFeature(s: S, feature: uint32_t) -> bool;
pub open spec fn FeatureGetFailed(s: S, feature: uint32_t) -> bool;
pub open spec fn FeatureConfigValue(s: S, feature: uint32_t) -> uint64_t;
pub open spec fn ResultEqual(a: SbiCommandReturnCode, b: SbiCommandReturnCode) -> bool;

} // verus!

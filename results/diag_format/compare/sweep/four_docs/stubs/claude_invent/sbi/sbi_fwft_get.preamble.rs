use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub fwft_values: Map<u32, u64>,
    pub fwft_supported: Set<u32>,
    pub fwft_implemented: Set<u32>,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_DENIED: i64 = -4;

pub open spec fn FwftFeatureIsReserved(feature: UInt32) -> bool;

pub open spec fn FwftFeatureIsPlatformSpecific(feature: UInt32) -> bool;

pub open spec fn FwftFeatureIsImplemented(s: S, feature: UInt32) -> bool;

pub open spec fn FwftFeatureIsValid(feature: UInt32) -> bool;

pub open spec fn FwftFeatureIsSupported(s: S, feature: UInt32) -> bool;

pub open spec fn FwftFeatureValue(s: S, feature: UInt32) -> UInt64;

} // verus!

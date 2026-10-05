use vstd::prelude::*;

verus! {

pub struct S {
    pub drtm_supported: bool,
    pub memory_protection_in_place: bool,
    pub dma_protection_complete: bool,
    pub dma_protection_region_based: bool,
    pub smmu_config: int,
}

pub const SUCCESS: i64 = 0;
pub const NOT_SUPPORTED: i64 = -1;
pub const DENIED: i64 = -3;

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn DrtmMemoryProtectionInPlace(s: S) -> bool;

pub open spec fn DrtmDmaProtectionIsComplete(s: S) -> bool;

pub open spec fn DrtmDmaProtectionIsRegionBased(s: S) -> bool;

pub open spec fn SmmuConfigEqual(old_s: S, new_s: S) -> bool;

pub open spec fn DrtmRegionProtectionsRemoved(old_s: S, new_s: S) -> bool;

} // verus!

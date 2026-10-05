use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub struct S {
    pub drtm_supported: bool,
    pub launch_memory_protections_in_place: bool,
    pub region_based_dma_protection: bool,
    pub complete_dma_protection: bool,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn IsDrtmSupported(s: S) -> bool;

pub open spec fn LaunchMemoryProtectionsInPlace(s: S) -> bool;

pub open spec fn IsRegionBasedDmaProtection(s: S) -> bool;

pub open spec fn IsCompleteDmaProtection(s: S) -> bool;

pub open spec fn SmmuConfigurationUnchanged(old_s: S, new_s: S) -> bool;

} // verus!

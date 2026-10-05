use vstd::prelude::*;

verus! {

pub enum NotSupported {
    NotSupportedError,
    DeniedError,
}

pub struct S {
    pub drtm_supported: bool,
    pub launch_memory_protections_in_place: bool,
    pub region_based_dma_protection: bool,
    pub complete_dma_protection: bool,
    pub smmu_configuration_unchanged: bool,
}

pub spec const SUCCESS: Result<(), NotSupported> = Ok(());

pub spec const NOT_SUPPORTED: Result<(), NotSupported> = Err(NotSupported::NotSupportedError);

pub spec const DENIED: Result<(), NotSupported> = Err(NotSupported::DeniedError);

pub open spec fn IsDrtmSupported(s: S) -> bool;

pub open spec fn LaunchMemoryProtectionsInPlace(s: S) -> bool;

pub open spec fn IsRegionBasedDmaProtection(s: S) -> bool;

pub open spec fn IsCompleteDmaProtection(s: S) -> bool;

pub open spec fn SmmuConfigurationUnchanged(s: S) -> bool;

pub open spec fn ResultEqual(a: Result<(), NotSupported>, b: Result<(), NotSupported>) -> bool;

} // verus!

use vstd::prelude::*;

verus! {

pub type ResultCode = u64;
pub type DmaProtectionKind = u32;
pub type SmmuConfig = u64;
pub type RegionList = Seq<u64>;

pub struct S {
    pub drtm_supported: bool,
    pub memory_protections_in_place: bool,
    pub dma_protection_type: DmaProtectionKind,
    pub smmu_configuration: SmmuConfig,
    pub protected_regions: RegionList,
}

pub const SUCCESS: ResultCode = 0;
pub const NOT_SUPPORTED: ResultCode = 1;
pub const DENIED: ResultCode = 2;

pub const COMPLETE: DmaProtectionKind = 0;
pub const REGION_BASED: DmaProtectionKind = 1;

pub open spec fn ResultValue() -> ResultCode;

pub spec const result: ResultCode = ResultValue();

pub open spec fn IsDrtmSupported(s: S) -> bool;

pub open spec fn ResultEqual(a: ResultCode, b: ResultCode) -> bool;

pub open spec fn MemoryProtectionsInPlace(s: S) -> bool;

pub open spec fn DmaProtectionType(s: S) -> DmaProtectionKind;

pub open spec fn SmmuConfiguration(s: S) -> SmmuConfig;

pub open spec fn SmmuConfiguration_pre(s: S) -> SmmuConfig;

pub open spec fn RegionProtectionsInPlace(s: S, regions: RegionList) -> bool;

pub open spec fn DrtmParametersProtectedRegions(s: S) -> RegionList;

} // verus!

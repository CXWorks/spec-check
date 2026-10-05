use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type UInt64 = u64;

pub struct S {
    pub drtm_supported: bool,
    pub memory_protections_in_place: bool,
    pub smmu_configuration: UInt64,
}

pub struct ProtectedRegion {
    pub base: UInt64,
    pub size: UInt64,
}

pub type DmaProtection = u64;

pub type SmmuConfig = u64;

pub spec const SUCCESS: Int64 = 0;

pub spec const NOT_SUPPORTED: Int64 = (-1) as i64;

pub spec const DENIED: Int64 = (-3) as i64;

pub spec const COMPLETE: DmaProtection = 0;

pub spec const REGION_BASED: DmaProtection = 1;

pub open spec fn IsDrtmSupported(s: S) -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn MemoryProtectionsInPlace(s: S) -> bool;

pub open spec fn DmaProtectionType() -> DmaProtection;

pub open spec fn SmmuConfiguration(s: S) -> SmmuConfig;

pub open spec fn RegionProtectionsInPlace(s: S, regions: Seq<ProtectedRegion>) -> bool;

pub open spec fn DrtmParametersProtectedRegions() -> Seq<ProtectedRegion>;

} // verus!

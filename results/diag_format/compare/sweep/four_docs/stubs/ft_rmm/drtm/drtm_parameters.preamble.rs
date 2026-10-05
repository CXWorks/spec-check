use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type RmiStatusCode = u64;

pub const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub struct LaunchFeatures {
    pub bits: u64,
}

impl LaunchFeatures {
    pub open spec fn spec_index<A>(self, i: A) -> int;
}

pub struct DRTM_PARAMETERS {
    pub revision: u64,
    pub reserved: u64,
    pub launch_features: LaunchFeatures,
    pub dlme_region_addr: u64,
    pub dlme_region_size: u64,
    pub dlme_image_start: u64,
    pub dlme_image_size: u64,
    pub dlme_entry_offset: u64,
    pub nw_dce_region_addr: u64,
    pub nw_dce_region_size: u64,
    pub mem_prot_table_addr: u64,
    pub mem_prot_table_size: u64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn IsNonSecure<A>(s: S, a: A) -> bool;

pub open spec fn IsPhysicallyContiguous<A>(s: S, a: A) -> bool;

pub open spec fn Is4KBAligned<A>(s: S, a: A) -> bool;

pub open spec fn AddressRangesOverlap(s: S, params: DRTM_PARAMETERS) -> bool;

pub open spec fn AnyAddressRangeWraps(s: S, params: DRTM_PARAMETERS) -> bool;

pub open spec fn DlmeImageWithinRegion(s: S, image_start: u64, image_size: u64, region_size: u64) -> bool;

pub open spec fn DlmeRegionMeetsRequirements(s: S, region_addr: u64, region_size: u64) -> bool;

pub open spec fn NwDceInUse(s: S, params: DRTM_PARAMETERS) -> bool;

} // verus!

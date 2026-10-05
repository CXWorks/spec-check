use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type RmiStatusCode = u64;

pub const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub struct DrtmParameters {
    pub revision: u64,
    pub reserved: u64,
    pub launch_features: Seq<u64>,
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
    pub drtm_parameters: DrtmParameters,
}

pub open spec fn IsNonSecure<T>(x: T) -> bool;

pub open spec fn IsPhysicallyContiguous<T>(x: T) -> bool;

pub open spec fn Is4KBAligned<T>(x: T) -> bool;

pub open spec fn AddressRangesOverlap(p: DrtmParameters) -> bool;

pub open spec fn AnyAddressRangeWraps(p: DrtmParameters) -> bool;

pub open spec fn DlmeImageWithinRegion(image_start: u64, image_size: u64, region_size: u64) -> bool;

pub open spec fn DlmeRegionMeetsRequirements(region_addr: u64, region_size: u64) -> bool;

pub open spec fn NwDceInUse(p: DrtmParameters) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

} // verus!

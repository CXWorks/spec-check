use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type RmiStatusCode = u64;

pub spec const RMI_SUCCESS: RmiStatusCode = 0;
pub spec const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub struct DRTMParameters {
    pub revision: u64,
    pub reserved: u64,
    pub launch_features: u64,
    pub dlme_region_address: u64,
    pub dlme_region_size: u64,
    pub dlme_image_start: u64,
    pub dlme_image_size: u64,
    pub dlme_entry_point_offset: u64,
    pub nwd_dce_region_address: u64,
    pub nwd_dce_region_size: u64,
    pub mpt_address: u64,
    pub mpt_size: u64,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn Bits(value: u64, hi: int, lo: int) -> int;

pub open spec fn IsNonSecureContiguous(addr: u64, size: u64) -> bool;

pub open spec fn ParamsAddr(params: DRTMParameters) -> u64;

pub open spec fn DrtmParametersSize(params: DRTMParameters) -> u64;

pub open spec fn IsAligned(addr: u64, alignment: int) -> bool;

pub open spec fn ParameterRangesOverlap(params: DRTMParameters) -> bool;

pub open spec fn ParameterRangesWrap(params: DRTMParameters) -> bool;

pub open spec fn DlmeRegionIsValid(addr: u64, size: u64) -> bool;

pub open spec fn NwdDceInUse(params: DRTMParameters) -> bool;

} // verus!

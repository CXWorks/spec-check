use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub struct DrtmParameters {
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

pub open spec fn DynamicLaunchFails(s: S, params: DrtmParameters) -> bool;

pub open spec fn Bits(x: u64, hi: int, lo: int) -> u64;

pub open spec fn IsNonSecureContiguous(s: S, addr: u64, size: u64) -> bool;

pub open spec fn ParamsAddr(params: DrtmParameters) -> u64;

pub open spec fn DrtmParametersSize(params: DrtmParameters) -> u64;

pub open spec fn IsAligned(s: S, addr: u64, alignment: u64) -> bool;

pub open spec fn ParameterRangesOverlap(s: S, params: DrtmParameters) -> bool;

pub open spec fn ParameterRangesWrap(s: S, params: DrtmParameters) -> bool;

pub open spec fn DlmeRegionIsValid(s: S, addr: u64, size: u64) -> bool;

pub open spec fn NwdDceInUse(s: S, params: DrtmParameters) -> bool;

} // verus!

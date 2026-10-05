use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsNonSecureContiguous(s: S, addr: int, size: int) -> bool;

pub open spec fn DlmeRegionMeetsRequirements(s: S, region_addr: int, region_size: int, image_start: int, image_size: int, data_offset: int) -> bool;

} // verus!

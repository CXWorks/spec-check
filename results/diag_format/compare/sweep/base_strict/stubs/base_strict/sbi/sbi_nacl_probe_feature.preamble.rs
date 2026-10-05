use vstd::prelude::*;

verus! {

pub type UInt = u64;
pub type UInt32 = u32;
pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub struct S {
    pub nacl_enabled: bool,
    pub shmem_addr: UInt,
}

pub open spec fn ResultEqual(a: SbiErrorCode, b: SbiErrorCode) -> bool;

pub open spec fn IsNaclFeatureAvailable(feature_id: UInt32) -> bool;

} // verus!

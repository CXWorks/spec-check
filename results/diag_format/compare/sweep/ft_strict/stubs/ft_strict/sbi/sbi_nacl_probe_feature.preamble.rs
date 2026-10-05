use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt = u64;
pub type SbiErrorCode = i64;

pub const SBI_SUCCESS: SbiErrorCode = 0;

pub struct S {
    pub nacl_features: Seq<bool>,
}

pub open spec fn IsNaclFeatureAvailable(s: S, feature_id: UInt32) -> bool;

} // verus!

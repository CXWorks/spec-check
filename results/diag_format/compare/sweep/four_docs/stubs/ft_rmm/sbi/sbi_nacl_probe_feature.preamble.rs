use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type SbiReturnCode = i64;

pub const SBI_SUCCESS: SbiReturnCode = 0;

pub struct S {
    pub nacl_features: Seq<bool>,
}

pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

pub open spec fn IsNaclFeatureAvailable(s: S, feature_id: UInt32) -> bool;

} // verus!

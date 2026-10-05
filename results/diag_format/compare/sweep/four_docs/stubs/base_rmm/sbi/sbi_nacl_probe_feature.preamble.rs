use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type UInt32 = u32;
pub type SbiReturnCode = i64;

pub const SBI_SUCCESS: SbiReturnCode = 0;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsNaclFeatureAvailable(feature_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

} // verus!

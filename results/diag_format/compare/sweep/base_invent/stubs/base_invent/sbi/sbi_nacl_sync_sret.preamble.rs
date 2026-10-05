use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub nacl_features: Set<u64>,
}

pub const SBI_NACL_FEAT_SYNC_SRET: u64 = 1;

pub open spec fn SbiFeatureAvailable(s: S, feature: u64) -> bool;

} // verus!

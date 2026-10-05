use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type SbiError = i64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub enum RmmFeature {
    FALSE,
    TRUE,
}

pub struct ImplFeatures {
    pub feat_sync_hfence: RmmFeature,
}

pub struct S {
    pub cmd_input_entry_index: u64,
    pub xlen: u32,
    pub impl_features: ImplFeatures,
    pub nacl_shmem_available: bool,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_NO_SHMEM: i64 = -9;

} // verus!

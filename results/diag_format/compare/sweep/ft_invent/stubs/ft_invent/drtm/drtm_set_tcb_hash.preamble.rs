use vstd::prelude::*;

verus! {

pub type X1 = u64;

pub type UInt64 = u64;

pub type DrtmSetTcbHashReturnCode = u64;

pub const RSI_SUCCESS: DrtmSetTcbHashReturnCode = 0;
pub const RSI_ERROR_NOT_SUPPORTED: DrtmSetTcbHashReturnCode = 1;
pub const RSI_INVALID_PARAMETERS: DrtmSetTcbHashReturnCode = 2;
pub const RSI_INVALID_DATA: DrtmSetTcbHashReturnCode = 3;
pub const RSI_OUT_OF_RESOURCE: DrtmSetTcbHashReturnCode = 4;
pub const RSI_DENIED: DrtmSetTcbHashReturnCode = 5;

pub struct DrtmFeatures {
    pub drtm_supported: bool,
    pub max_tcb_hash_table_entries: UInt64,
}

pub struct S {
    pub drtm_features: DrtmFeatures,
}

pub open spec fn DRTM_FEATURES(s: S) -> DrtmFeatures;

} // verus!

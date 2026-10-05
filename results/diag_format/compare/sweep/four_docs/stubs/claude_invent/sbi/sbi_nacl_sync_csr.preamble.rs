use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type Int64 = i64;

pub struct SbiRet {
    pub error: Int64,
    pub value: UInt64,
}

pub struct S {
    pub dummy: int,
}

pub const SBI_SUCCESS: Int64 = 0;
pub const SBI_ERR_FAILED: Int64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: Int64 = -2;
pub const SBI_ERR_INVALID_PARAM: Int64 = -3;
pub const SBI_ERR_DENIED: Int64 = -4;
pub const SBI_ERR_INVALID_ADDRESS: Int64 = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: Int64 = -6;
pub const SBI_ERR_ALREADY_STARTED: Int64 = -7;
pub const SBI_ERR_ALREADY_STOPPED: Int64 = -8;
pub const SBI_ERR_NO_SHMEM: Int64 = -9;

pub const SBI_NACL_FEAT_SYNC_CSR: UInt64 = 0;
pub const SBI_NACL_FEAT_SYNC_HFENCE: UInt64 = 1;
pub const SBI_NACL_FEAT_SYNC_SRET: UInt64 = 2;
pub const SBI_NACL_FEAT_AUTOSWAP_CSR: UInt64 = 3;

pub open spec fn NaclFeatureAvailable(s: S, feature: UInt64) -> bool;

pub open spec fn NaclShmemAvailable(s: S) -> bool;

pub open spec fn IsHCsrImplemented(s: S, csr_num: UInt64) -> bool;

pub open spec fn NaclAllHCsrsSynchronized(old_s: S, new_s: S) -> bool;

pub open spec fn NaclHCsrSynchronized(old_s: S, new_s: S, csr_num: UInt64) -> bool;

} // verus!

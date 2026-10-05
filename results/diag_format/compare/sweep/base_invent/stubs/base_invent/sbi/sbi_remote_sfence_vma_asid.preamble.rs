use vstd::prelude::*;

verus! {

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub dummy: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub const start_addr: u64 = 4096;
pub const size: u64 = 8192;
pub const asid: u64 = 1;
pub const hart_mask: u64 = 3;
pub const hart_mask_base: u64 = 5;

} // verus!

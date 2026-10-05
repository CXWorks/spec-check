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
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

pub open spec fn hart_mask_val() -> u64;
pub open spec fn hart_mask_base_val() -> u64;
pub open spec fn start_addr_val() -> u64;
pub open spec fn size_val() -> u64;

pub spec const hart_mask: u64 = hart_mask_val();
pub spec const hart_mask_base: u64 = hart_mask_base_val();
pub spec const start_addr: u64 = start_addr_val();
pub spec const size: u64 = size_val();

} // verus!

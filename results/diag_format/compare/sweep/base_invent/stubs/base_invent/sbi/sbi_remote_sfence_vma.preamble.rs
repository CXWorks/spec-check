use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub hart_mask: u64,
    pub hart_mask_base: u64,
    pub start_addr: u64,
    pub size: u64,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;

} // verus!

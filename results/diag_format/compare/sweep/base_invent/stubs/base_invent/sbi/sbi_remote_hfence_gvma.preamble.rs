use vstd::prelude::*;
verus! {

pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiCommandReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;
pub const SBI_ERR_INVALID_ADDRESS: SbiCommandReturnCode = -5;

pub struct S {
    pub hypervisor_extension_supported: bool,
    pub hart_mask: u64,
    pub hart_mask_base: u64,
    pub start_addr: int,
    pub size: int,
}

impl S {
    pub open spec fn all_harts_in_mask_valid(&self, hart_mask: u64, hart_mask_base: u64) -> bool;
}

} // verus!

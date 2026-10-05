use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Int64 = i64;

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;

pub struct sbiret {
    pub error: i64,
    pub value: i64,
}

pub struct S {
    pub hart_mask: u64,
    pub hart_mask_base: u64,
    pub num_harts: u64,
    pub pending_ipi: Seq<bool>,
}

pub open spec fn hart_mask_invalid(s: S) -> bool;

pub open spec fn request_failed(old_s: S, new_s: S) -> bool;

pub open spec fn ipi_sent_to_valid_harts(old_s: S, new_s: S) -> bool;

} // verus!

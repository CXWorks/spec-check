use vstd::prelude::*;
verus! {

pub type sbiret = i64;

pub const SBI_SUCCESS: sbiret = 0;
pub const SBI_ERR_FAILED: sbiret = -1;
pub const SBI_ERR_ALREADY_STARTED: sbiret = -7;

pub struct S {
    pub hart_id: u64,
}

impl S {
    pub open spec fn sse_hart_unmasked(&self, hart_id: u64) -> bool;
}

} // verus!

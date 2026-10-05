use vstd::prelude::*;
verus! {

pub type sbiret = i64;

pub const SBI_SUCCESS: sbiret = 0;
pub const SBI_ERR_FAILED: sbiret = -1;
pub const SBI_ERR_ALREADY_STOPPED: sbiret = -8;

pub struct S {
    pub hart_masked: bool,
}

} // verus!

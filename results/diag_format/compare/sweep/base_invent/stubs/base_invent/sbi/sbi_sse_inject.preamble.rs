use vstd::prelude::*;
verus! {

pub type sbiret = i64;

pub const SBI_SUCCESS: sbiret = 0;
pub const SBI_ERR_FAILED: sbiret = -1;
pub const SBI_ERR_NOT_SUPPORTED: sbiret = -2;
pub const SBI_ERR_INVALID_PARAM: sbiret = -3;
pub const SBI_ERR_DENIED: sbiret = -4;
pub const SBI_ERR_INVALID_ADDRESS: sbiret = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: sbiret = -6;
pub const SBI_ERR_ALREADY_STARTED: sbiret = -7;
pub const SBI_ERR_ALREADY_STOPPED: sbiret = -8;

pub struct S {
    pub dummy: u64,
}

} // verus!

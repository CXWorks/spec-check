use vstd::prelude::*;
verus! {

pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub const SBI_ERR_ALREADY_STOPPED: SbiCommandReturnCode = -8;

pub struct S {
    pub dummy: u64,
}

} // verus!

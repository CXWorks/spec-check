use vstd::prelude::*;
verus! {

pub type SbiError = i64;

pub struct sbiret {
    pub error: SbiError,
    pub value: i64,
    pub uvalue: u64,
}

pub struct S {
    pub console_output: Seq<u8>,
}

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_FAILED: SbiError = -1;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub const SBI_ERR_DENIED: SbiError = -4;

#[allow(non_upper_case_globals)]
pub const num_bytes: i64 = 10;
#[allow(non_upper_case_globals)]
pub const base_addr_lo: i64 = 11;
#[allow(non_upper_case_globals)]
pub const base_addr_hi: i64 = 12;

} // verus!

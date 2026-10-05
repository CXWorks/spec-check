use vstd::prelude::*;

verus! {

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub sse_event_id: u64,
    pub sse_base_attr_id: u64,
    pub sse_attr_count: u64,
    pub sse_input_phys_lo: u64,
    pub sse_input_phys_hi: u64,
    pub xlen: u64,
    pub sse_supported: bool,
    pub sse_readonly: bool,
    pub sse_state_valid: bool,
    pub sse_range_valid: bool,
    pub sse_attr_values_valid: bool,
    pub sse_attr_values_readonly: bool,
    pub sse_attr_values_state_invalid: bool,
    pub sse_attr_values_range_invalid: bool,
    pub sse_attr_values_failed: bool,
}

pub const SBI_SUCCESS: i64 = 0;
pub const SBI_ERR_FAILED: i64 = -1;
pub const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub const SBI_ERR_INVALID_PARAM: i64 = -3;
pub const SBI_ERR_DENIED: i64 = -4;
pub const SBI_ERR_INVALID_ADDRESS: i64 = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: i64 = -6;
pub const SBI_ERR_ALREADY_STARTED: i64 = -7;
pub const SBI_ERR_ALREADY_STOPPED: i64 = -8;
pub const SBI_ERR_NO_SHMEM: i64 = -9;
pub const SBI_ERR_INVALID_STATE: i64 = -10;
pub const SBI_ERR_BAD_RANGE: i64 = -11;
pub const SBI_ERR_TIMEOUT: i64 = -12;
pub const SBI_ERR_IO: i64 = -13;
pub const SBI_ERR_DENIED_LOCKED: i64 = -14;

} // verus!

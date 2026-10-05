use vstd::prelude::*;
verus! {

pub type FfaReturnCode = i64;

pub const FFA_SUCCESS: FfaReturnCode = 0;
pub const FFA_ERROR: FfaReturnCode = 1;
pub const FFA_ERROR_BUSY: FfaReturnCode = 2;
pub const FFA_ERROR_INVALID_PARAMETERS: FfaReturnCode = 3;
pub const FFA_ERROR_NO_MEMORY: FfaReturnCode = 4;
pub const FFA_ERROR_DENIED: FfaReturnCode = 5;
pub const FFA_ERROR_NOT_SUPPORTED: FfaReturnCode = 6;
pub const FFA_ERROR_NOT_READY: FfaReturnCode = 7;

pub struct S {
    pub dummy: u64,
}

impl S {
    pub open spec fn ff_a_rx_buffer_is_free(self) -> bool;
    pub open spec fn uuid_is_valid(self) -> bool;
    pub open spec fn rx_buffer_can_hold_descriptors(self) -> bool;
    pub open spec fn callee_state_can_handle(self) -> bool;
    pub open spec fn instance_supports_function(self) -> bool;
    pub open spec fn callee_is_ready(self) -> bool;
    pub open spec fn flags_bit0_is_set(self) -> bool;
    pub open spec fn flags_bit0_is_clear(self) -> bool;
    pub open spec fn count(self) -> int;
    pub open spec fn size(self) -> int;
    pub open spec fn descriptors_populated(self) -> bool;
}

pub open spec fn ResultEqual(a: FfaReturnCode, b: FfaReturnCode) -> bool;

} // verus!

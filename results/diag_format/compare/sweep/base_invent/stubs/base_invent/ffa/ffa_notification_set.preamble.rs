use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint64 = u64;

pub struct Registers {
    pub w0: uint32,
    pub w1: uint32,
    pub w2: uint32,
    pub w3: uint32,
    pub w4: uint32,
    pub w5: uint32,
    pub w6: uint32,
    pub w7: uint32,
}

pub struct S {
    pub registers: Registers,
    pub per_vcpu_notifications_supported: bool,
}

pub const FFA_SUCCESS: int32 = 0;
pub const FFA_ERROR_NOT_SUPPORTED: int32 = -1;
pub const FFA_ERROR_INVALID_PARAMETERS: int32 = -2;
pub const FFA_ERROR_NO_MEMORY: int32 = -3;
pub const FFA_ERROR_BUSY: int32 = -4;
pub const FFA_ERROR_INTERRUPTED: int32 = -5;
pub const FFA_ERROR_DENIED: int32 = -6;
pub const FFA_ERROR_RETRY: int32 = -7;
pub const FFA_ERROR_ABORTED: int32 = -8;
pub const FFA_ERROR_NO_DATA: int32 = -9;

pub uninterp spec fn ResultEqual(result: int32, expected: int32) -> spec_fn(bool) -> bool;

} // verus!

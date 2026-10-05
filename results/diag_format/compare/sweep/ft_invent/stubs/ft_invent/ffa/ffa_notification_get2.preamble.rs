use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;

pub enum FfaReturn {
    FfaSuccess { w2: UInt64, w3: UInt64, w4: UInt64, w5: UInt64, w6: UInt64, w7: UInt64 },
    FfaError { code: Int32 },
}

pub struct S {
    pub notifications_enabled: bool,
}

pub const FFA_ERROR_NOT_SUPPORTED: Int32 = -1;
pub const FFA_ERROR_INVALID_PARAMETERS: Int32 = -2;
pub const FFA_ERROR_NO_MEMORY: Int32 = -3;
pub const FFA_ERROR_BUSY: Int32 = -4;
pub const FFA_ERROR_INTERRUPTED: Int32 = -5;
pub const FFA_ERROR_DENIED: Int32 = -6;
pub const FFA_ERROR_RETRY: Int32 = -7;
pub const FFA_ERROR_ABORTED: Int32 = -8;
pub const FFA_ERROR_NO_DATA: Int32 = -9;

pub open spec fn FFA_ERROR(code: Int32) -> FfaReturn;

} // verus!

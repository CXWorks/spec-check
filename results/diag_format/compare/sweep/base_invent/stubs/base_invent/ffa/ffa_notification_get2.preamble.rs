use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FfaReturnCode = i64;

pub struct FfaState {
    pub notifications_pending: Seq<UInt64>,
    pub sp_notifications: Seq<UInt64>,
    pub vm_notifications: Seq<UInt64>,
    pub spmc_notifications: UInt64,
    pub hypervisor_notifications: UInt64,
}

pub const FFA_SUCCESS: FfaReturnCode = 0;
pub const FFA_ERROR_NOT_SUPPORTED: FfaReturnCode = -1;
pub const FFA_ERROR_INVALID_PARAMETERS: FfaReturnCode = -2;
pub const FFA_ERROR_NO_MEMORY: FfaReturnCode = -3;
pub const FFA_ERROR_BUSY: FfaReturnCode = -4;
pub const FFA_ERROR_INTERRUPTED: FfaReturnCode = -5;
pub const FFA_ERROR_DENIED: FfaReturnCode = -6;
pub const FFA_ERROR_RETRY: FfaReturnCode = -7;
pub const FFA_ERROR_ABORTED: FfaReturnCode = -8;
pub const FFA_ERROR_NO_DATA: FfaReturnCode = -9;

pub open spec fn ResultEqual(result: FfaReturnCode, expected: FfaReturnCode) -> bool;

} // verus!

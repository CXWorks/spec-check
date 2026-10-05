use vstd::prelude::*;

verus! {

pub type int32 = i32;

pub const FFA_SUCCESS: int32 = 0;
pub const FFA_ERROR_NOT_SUPPORTED: int32 = -1;
pub const FFA_ERROR_INVALID_PARAMETERS: int32 = -2;
pub const FFA_ERROR_DENIED: int32 = -6;
pub const FFA_ERROR_ABORTED: int32 = -8;

pub struct S {
    pub sender_id: u32,
    pub receiver_id: u32,
    pub flags: u32,
    pub per_vcpu_notifications_supported: bool,
    pub notifications_bound: Set<u32>,
    pub notifications_pending: Set<u32>,
    pub caller_allowed_to_invoke_abi: bool,
    pub sender_partition_aborted: bool,
    pub notification_bitmap: u64,
}

} // verus!

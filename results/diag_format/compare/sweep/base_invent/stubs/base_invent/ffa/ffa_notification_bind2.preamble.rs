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
    pub per_vcpu_supported: bool,
    pub notification_bitmap_count: u32,
    pub max_notifications: int,
    pub ffa_notification_bind2_supported: bool,
    pub notification_bound: bool,
    pub notification_pending: bool,
    pub caller_allowed: bool,
    pub sender_aborted: bool,
    pub notification_bitmap: u64,
}

} // verus!

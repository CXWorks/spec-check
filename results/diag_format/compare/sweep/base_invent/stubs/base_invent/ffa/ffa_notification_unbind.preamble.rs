use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint64 = u64;

pub struct S {
    pub sender_id: uint32,
    pub receiver_id: uint32,
    pub notification_bitmap: uint64,
}

pub const FFA_SUCCESS: int32 = 0;
pub const FFA_ERROR_NOT_SUPPORTED: int32 = -1;
pub const FFA_ERROR_INVALID_PARAMETERS: int32 = -2;
pub const FFA_ERROR_DENIED: int32 = -6;
pub const FFA_ERROR_ABORTED: int32 = -8;

pub open spec fn IsPartitionIdValid(s: S, id: uint32) -> bool;
pub open spec fn IsNotificationBitmapValid(s: S) -> bool;
pub open spec fn ExistsBoundNotification(s: S, sender_id: uint32, bitmap: uint64) -> bool;
pub open spec fn ExistsPendingNotification(s: S, sender_id: uint32, bitmap: uint64) -> bool;
pub open spec fn IsCallerAllowed(s: S, sender_id: uint32) -> bool;
pub open spec fn IsPartitionAborted(s: S, sender_id: uint32) -> bool;
pub open spec fn ExtractSenderId(s: S) -> uint32;
pub open spec fn ExtractReceiverId(s: S) -> uint32;
pub open spec fn ExtractNotificationBitmap(s: S) -> uint64;

} // verus!

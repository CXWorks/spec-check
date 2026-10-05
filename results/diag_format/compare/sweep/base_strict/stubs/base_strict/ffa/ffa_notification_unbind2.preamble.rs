use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type PartitionId = UInt16;

pub struct S {
    pub dummy: UInt64,
}

pub struct FfaInstance {
    pub id: UInt64,
}

pub struct Caller {
    pub id: PartitionId,
}

pub const FFA_SUCCESS: UInt32 = 0;
pub const NOT_SUPPORTED: UInt32 = 1;
pub const INVALID_PARAMETERS: UInt32 = 2;
pub const DENIED: UInt32 = 3;
pub const ABORTED: UInt32 = 4;
pub const FFA_NOTIFICATION_UNBIND2: UInt32 = 0x8400_0080;

pub const sender: PartitionId = 1;
pub const receiver: PartitionId = 2;

pub open spec fn IsImplementedAtInstance(func_id: UInt32, inst: FfaInstance) -> bool;

pub open spec fn CurrentFfaInstance() -> FfaInstance;

pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;

pub open spec fn IsValidPartitionId(id: PartitionId) -> bool;

pub open spec fn IsValidNotificationBitmap(bitmap: [UInt64; 8]) -> bool;

pub open spec fn NotificationBitmapBit(bitmap: [UInt64; 8], i: UInt64) -> UInt64;

pub open spec fn NumSupportedNotifications() -> UInt64;

pub open spec fn NotificationIsBound(recv: PartitionId, i: UInt64) -> bool;

pub open spec fn NotificationBoundSender(recv: PartitionId, i: UInt64) -> PartitionId;

pub open spec fn NotificationIsPending(recv: PartitionId, i: UInt64) -> bool;

pub open spec fn CallerMayInvokeNotificationUnbind2(caller: Caller) -> bool;

pub open spec fn CurrentCaller() -> Caller;

pub open spec fn PartitionHasAborted(id: PartitionId) -> bool;

pub open spec fn SenderCanSignalNotification(snd: PartitionId, recv: PartitionId, i: UInt64) -> bool;

pub open spec fn NotificationBinding(recv: PartitionId, i: UInt64) -> UInt64;

pub open spec fn OldNotificationBinding(recv: PartitionId, i: UInt64) -> UInt64;

} // verus!

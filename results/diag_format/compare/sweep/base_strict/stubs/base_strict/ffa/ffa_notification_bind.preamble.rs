use vstd::prelude::*;

verus! {

pub type NotificationId = u32;

pub type FfaFunctionId = u32;

pub type PartitionId = u16;

pub enum FfaInstance {
    Physical,
    Virtual,
}

pub struct S {
    pub sender_id: PartitionId,
    pub receiver_id: PartitionId,
    pub flags: u32,
    pub notification_bitmap_lo: u32,
    pub notification_bitmap_hi: u32,
}

pub const FFA_NOTIFICATION_BIND: FfaFunctionId = 0x8400007F;

pub const FFA_SUCCESS: u32 = 0;

pub const NOT_SUPPORTED: u32 = 1;

pub const INVALID_PARAMETERS: u32 = 2;

pub const DENIED: u32 = 3;

pub const ABORTED: u32 = 4;

pub open spec fn IsImplementedAtInstance(func: FfaFunctionId, inst: FfaInstance) -> bool;

pub open spec fn CurrentFfaInstance() -> FfaInstance;

pub open spec fn ResultEqual(result: u32, code: u32) -> bool;

pub open spec fn IsValidSenderId(id: PartitionId) -> bool;

pub open spec fn IsValidReceiverId(id: PartitionId) -> bool;

pub open spec fn Bits(value: u32, hi: int, lo: int) -> u32;

pub open spec fn PerVcpuNotificationsSupported() -> bool;

pub open spec fn BitmapBitSet(lo: u32, hi: u32, n: NotificationId) -> bool;

pub open spec fn IsBoundToOtherSender(receiver: PartitionId, n: NotificationId, sender: PartitionId) -> bool;

pub open spec fn IsNotificationPending(receiver: PartitionId, n: NotificationId) -> bool;

pub open spec fn CallerAllowedToInvoke(func: FfaFunctionId) -> bool;

pub open spec fn PartitionHasAborted(id: PartitionId) -> bool;

pub open spec fn IsBoundToSender(receiver: PartitionId, n: NotificationId, sender: PartitionId) -> bool;

pub open spec fn IsPerVcpuNotification(receiver: PartitionId, n: NotificationId) -> bool;

} // verus!

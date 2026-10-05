use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type PartitionId = u32;
pub type NotificationId = int;
pub type FunctionId = u32;
pub type ReturnCode = u32;
pub type FfaInstance = int;
pub type BindingValue = u32;

pub struct S {
    pub dummy: int,
}

pub const FFA_NOTIFICATION_UNBIND: FunctionId = 0x8400007F;

pub const FFA_SUCCESS: ReturnCode = 0;
pub const NOT_SUPPORTED: ReturnCode = 1;
pub const INVALID_PARAMETERS: ReturnCode = 2;
pub const DENIED: ReturnCode = 3;
pub const ABORTED: ReturnCode = 4;

pub open spec fn IsImplementedAtInstance(func: FunctionId, inst: FfaInstance) -> bool;

pub open spec fn CurrentFfaInstance() -> FfaInstance;

pub open spec fn ResultEqual(result: u32, code: ReturnCode) -> bool;

pub open spec fn Bits(value: u32, hi: int, lo: int) -> PartitionId;

pub open spec fn IsValidPartitionId(id: PartitionId) -> bool;

pub open spec fn IsValidNotificationBitmap(lo: u32, hi: u32) -> bool;

pub open spec fn IsCallerAllowedToInvoke(func: FunctionId) -> bool;

pub open spec fn Bitmap64(hi: u32, lo: u32) -> UInt64;

pub open spec fn IsBitSet(bitmap: UInt64, n: NotificationId) -> bool;

pub open spec fn IsNotificationBound(receiver: PartitionId, n: NotificationId) -> bool;

pub open spec fn BoundSender(receiver: PartitionId, n: NotificationId) -> PartitionId;

pub open spec fn IsNotificationPending(receiver: PartitionId, n: NotificationId) -> bool;

pub open spec fn HasPartitionAborted(id: PartitionId) -> bool;

pub open spec fn CanSenderSignal(sender: PartitionId, receiver: PartitionId, n: NotificationId) -> bool;

pub open spec fn NotificationBinding(receiver: PartitionId, n: NotificationId) -> BindingValue;

pub open spec fn PreNotificationBinding(receiver: PartitionId, n: NotificationId) -> BindingValue;

} // verus!

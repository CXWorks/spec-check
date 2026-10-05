use vstd::prelude::*;

verus! {

pub type UInt = u64;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FuncId = u32;
pub type Instance = u8;
pub type Flags = u16;
pub type Bitmap = u64;

pub type Result = i32;

pub const FFA_SUCCESS: Result = 0;
pub const NOT_SUPPORTED: Result = -1;
pub const INVALID_PARAMETERS: Result = -2;
pub const DENIED: Result = -6;
pub const ABORTED: Result = -8;

pub const FFA_NOTIFICATION_SET: FuncId = 0x8400_0081;

pub const instance: Instance = 1;
pub const flags: Flags = 2;
pub const bitmap: Bitmap = 3;

pub struct S {
    pub dummy: int,
}

pub struct NotificationStateT {
    pub global: Bitmap,
    pub per_vcpu: Bitmap,
}

pub struct SriState {
    pub pending: bool,
}

pub open spec fn IsImplementedAtInstance(s: S, func: FuncId, inst: Instance) -> bool;

pub open spec fn ResultEqual(a: Result, b: Result) -> bool;

pub open spec fn IsRecognizedPartitionId(s: S, id: UInt16) -> bool;

pub open spec fn AreValidFlags(s: S, f: Flags, inst: Instance) -> bool;

pub open spec fn BitmapHasPerVcpuNotification(s: S, receiver_id: UInt16, b: Bitmap) -> bool;

pub open spec fn BitmapHasGlobalNotification(s: S, receiver_id: UInt16, b: Bitmap) -> bool;

pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;

pub open spec fn SenderMaySignalAll(s: S, sender_id: UInt16, receiver_id: UInt16, b: Bitmap) -> bool;

pub open spec fn ReceiverSupportsNotifications(s: S, receiver_id: UInt16) -> bool;

pub open spec fn ReceiverHasAborted(s: S, receiver_id: UInt16) -> bool;

pub open spec fn NotificationSignaled(s: S, receiver_id: UInt16, vcpu_id: UInt16, i: int) -> bool;

pub open spec fn NotificationState(s: S, receiver_id: UInt16, vcpu_id: int) -> NotificationStateT;

pub open spec fn ScheduleReceiverInterrupt(s: S) -> SriState;

} // verus!

use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt = u32;
pub type UInt64 = u64;
pub type Result = u32;
pub type FunctionId = u32;
pub type Bitmap = u128;
pub type Instance = u8;

pub struct S {
    pub dummy: int,
}

pub struct NotificationPending {
    pub global: u64,
    pub per_vcpu: Seq<u64>,
}

pub struct SriState {
    pub pending: bool,
    pub delayed: bool,
}

pub const FFA_SUCCESS: Result = 0;
pub const NOT_SUPPORTED: Result = 1;
pub const INVALID_PARAMETERS: Result = 2;
pub const DENIED: Result = 3;
pub const ABORTED: Result = 4;
pub const result_fid: Result = 5;

pub const FFA_NOTIFICATION_SET: FunctionId = 0x84000079;

pub open spec fn IsImplementedAtInstance(s: S, fid: FunctionId, inst: Instance) -> bool;
pub open spec fn CallerInstance(s: S) -> Instance;
pub open spec fn ResultEqual(a: Result, b: Result) -> bool;
pub open spec fn IsValidPartitionId(s: S, id: UInt16) -> bool;
pub open spec fn AreValidNotificationSetFlags(s: S, per_vcpu: UInt, delay_sri: UInt, flags_reserved: UInt, receiver_vcpu_id: UInt16, inst: Instance) -> bool;
pub open spec fn NotificationBitmap(lo: UInt64, hi: UInt64) -> Bitmap;
pub open spec fn IsBitSet(s: S, bitmap: Bitmap, n: UInt64) -> bool;
pub open spec fn IsPerVcpuNotification(s: S, receiver_id: UInt16, n: UInt64) -> bool;
pub open spec fn IsGlobalNotification(s: S, receiver_id: UInt16, n: UInt64) -> bool;
pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;
pub open spec fn IsPermittedToSignal(s: S, sender_id: UInt16, receiver_id: UInt16, n: UInt64) -> bool;
pub open spec fn SupportsNotificationReceipt(s: S, receiver_id: UInt16) -> bool;
pub open spec fn HasAborted(s: S, receiver_id: UInt16) -> bool;
pub open spec fn IsGlobalNotificationPending(s: S, receiver_id: UInt16, n: UInt64) -> bool;
pub open spec fn IsPerVcpuNotificationPending(s: S, receiver_id: UInt16, vcpu_id: UInt16, n: UInt64) -> bool;
pub open spec fn IsSignaledByCall(s: S, receiver_id: UInt16, n: UInt64) -> bool;
pub open spec fn ScheduleReceiverInterruptPerImplDefinedPolicy(s: S, receiver_id: UInt16, delay_sri: UInt) -> bool;
pub open spec fn NotificationPendingState(s: S, receiver_id: UInt16) -> NotificationPending;
pub open spec fn ScheduleReceiverInterruptState(s: S) -> SriState;

} // verus!

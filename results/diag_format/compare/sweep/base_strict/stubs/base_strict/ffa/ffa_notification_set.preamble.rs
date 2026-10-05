use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type UInt16 = u16;

pub struct S {
    pub dummy: int,
}

pub type PartitionId = UInt16;
pub type InstanceId = UInt64;
pub type Bitmap = UInt64;

pub const FFA_NOTIFICATION_SET: UInt32 = 0x8400_0081u32;
pub const FFA_SUCCESS: UInt32 = 0x8400_0061u32;

pub const NOT_SUPPORTED: Int32 = -1i32;
pub const INVALID_PARAMETERS: Int32 = -2i32;
pub const DENIED: Int32 = -6i32;
pub const ABORTED: Int32 = -8i32;

pub open spec fn n() -> UInt64;

pub open spec fn IsImplementedAtInstance(s: S, fid: UInt32, inst: InstanceId) -> bool;
pub open spec fn CallerInstance(s: S) -> InstanceId;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsValidPartitionId(s: S, id: PartitionId) -> bool;
pub open spec fn sender_id(s: S) -> PartitionId;
pub open spec fn receiver_id(s: S) -> PartitionId;
pub open spec fn per_vcpu(s: S) -> UInt64;
pub open spec fn delay_sri(s: S) -> UInt64;
pub open spec fn flags_reserved(s: S) -> UInt64;
pub open spec fn receiver_vcpu_id(s: S) -> UInt64;
pub open spec fn bitmap_lo(s: S) -> UInt32;
pub open spec fn bitmap_hi(s: S) -> UInt32;
pub open spec fn AreValidNotificationSetFlags(s: S, per_vcpu: UInt64, delay_sri: UInt64, flags_reserved: UInt64, receiver_vcpu_id: UInt64, inst: InstanceId) -> bool;
pub open spec fn IsBitSet(s: S, bm: Bitmap, n: UInt64) -> bool;
pub open spec fn NotificationBitmap(s: S, lo: UInt32, hi: UInt32) -> Bitmap;
pub open spec fn IsPerVcpuNotification<T>(s: S, id: PartitionId, n: T) -> bool;
pub open spec fn IsGlobalNotification<T>(s: S, id: PartitionId, n: T) -> bool;
pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;
pub open spec fn IsPermittedToSignal<T>(s: S, sender: PartitionId, receiver: PartitionId, n: T) -> bool;
pub open spec fn SupportsNotificationReceipt(s: S, id: PartitionId) -> bool;
pub open spec fn HasAborted(s: S, id: PartitionId) -> bool;
pub open spec fn IsGlobalNotificationPending(s: S, id: PartitionId, n: UInt64) -> bool;
pub open spec fn IsPerVcpuNotificationPending(s: S, id: PartitionId, vcpu: UInt64, n: UInt64) -> bool;
pub open spec fn IsSignaledByCall(s: S, id: PartitionId, n: UInt64) -> bool;
pub open spec fn ScheduleReceiverInterruptPerImplDefinedPolicy(s: S, id: PartitionId, delay_sri: UInt64) -> bool;

} // verus!

use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;

pub struct S {
    pub dummy: UInt32,
}

pub struct NotificationStateT {
    pub global: UInt32,
    pub per_vcpu: UInt32,
}

pub struct InterruptT {
    pub pending: bool,
}

pub spec const FFA_NOTIFICATION_SET: UInt32 = 0x8400007Eu32;

pub spec const FFA_SUCCESS: UInt32 = 0u32;
pub spec const NOT_SUPPORTED: UInt32 = 1u32;
pub spec const INVALID_PARAMETERS: UInt32 = 2u32;
pub spec const DENIED: UInt32 = 3u32;
pub spec const ABORTED: UInt32 = 4u32;

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;
pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;
pub open spec fn IsRecognizedPartitionId(s: S, id: UInt16) -> bool;
pub open spec fn sender_id(s: S) -> UInt16;
pub open spec fn receiver_id(s: S) -> UInt16;
pub open spec fn flags(s: S) -> UInt32;
pub open spec fn instance(s: S) -> UInt32;
pub open spec fn AreValidFlags(s: S, flags: UInt32, instance: UInt32) -> bool;
pub open spec fn per_vcpu(s: S) -> UInt32;
pub open spec fn receiver_vcpu_id(s: S) -> UInt16;
pub open spec fn bitmap(s: S) -> Map<UInt32, UInt32>;
pub open spec fn BitmapHasPerVcpuNotification(s: S, receiver: UInt16, bitmap: Map<UInt32, UInt32>) -> bool;
pub open spec fn BitmapHasGlobalNotification(s: S, receiver: UInt16, bitmap: Map<UInt32, UInt32>) -> bool;
pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;
pub open spec fn SenderMaySignalAll(s: S, sender: UInt16, receiver: UInt16, bitmap: Map<UInt32, UInt32>) -> bool;
pub open spec fn ReceiverSupportsNotifications(s: S, receiver: UInt16) -> bool;
pub open spec fn ReceiverHasAborted(s: S, receiver: UInt16) -> bool;
pub open spec fn NotificationSignaled(s: S, receiver: UInt16, i: UInt32, new_s: S) -> bool;
pub open spec fn NotificationState(s: S, receiver: UInt16) -> NotificationStateT;
pub open spec fn ScheduleReceiverInterrupt(s: S) -> InterruptT;

} // verus!
